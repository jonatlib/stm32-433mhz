use core::borrow::Borrow;

use sequence_number::SequenceNumber;

use crate::packet::{Packet64, PacketKind};
use crate::Address;

pub struct PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8>,
{
    address: &'a Address,

    sequence_number: &'a mut SequenceNumber<32>,
    payload: P,

    last_byte: Option<u8>,
}

impl<'a, P, I> PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8>,
{
    pub fn new(
        address: &'a Address,
        start_sequence_number: &'a mut SequenceNumber<32>,
        mut payload: P,
    ) -> Self {
        Self {
            address,
            sequence_number: start_sequence_number,
            payload,

            last_byte: None,
        }
    }
}

impl<'a, P, I> Iterator for PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8> + Clone,
{
    type Item = Packet64;

    fn next(&mut self) -> Option<Self::Item> {
        let was_previous_byte_set = self.last_byte.is_some();
        let first_payload_byte = self
            .last_byte
            .or_else(|| self.payload.next().map(|v| v.borrow().clone()))?;
        self.last_byte = None;

        let packet = Packet64::new()
            .with_sequence_number(self.sequence_number.advance())
            .with_source_address(self.address.local_address)
            .with_destination_address(self.address.destination_address);

        let mut payload_buffer = [0x0u8; 8];
        payload_buffer[0] = first_payload_byte;

        for index in 1usize..5 {
            if let Some(byte) = self.payload.next() {
                payload_buffer[index] = byte.borrow().clone();
            } else {
                return Some(
                    packet
                        .with_kind(if was_previous_byte_set {
                            PacketKind::End
                        } else {
                            PacketKind::SelfContained
                        })
                        .with_payload(u64::from_le_bytes(payload_buffer))
                        .with_payload_used_index(index as u8)
                        .with_updated_crc(),
                );
            }
        }

        let probably_last_byte = self.payload.next().map(|v| v.borrow().clone());
        self.last_byte = probably_last_byte;

        Some(
            packet
                .with_kind(if was_previous_byte_set {
                    if probably_last_byte.is_some() {
                        PacketKind::Continue
                    } else {
                        PacketKind::End
                    }
                } else {
                    PacketKind::Start
                })
                .with_payload(u64::from_le_bytes(payload_buffer))
                .with_payload_used_index(4)
                .with_updated_crc(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::packet::{Packet64, PacketKind};
    use crate::Address;
    use sequence_number::SequenceNumber;

    use std::vec::Vec;

    #[test]
    fn test_base_packet_builder() {
        let paylaod = vec![0x01u8, 0x02, 0xab, 0xcd];
        let mut sequence_number = SequenceNumber::new(1);
        let address = Address::new(0x01, 0x02);
        let builder =
            PacketBuilder::new(&address, &mut sequence_number, paylaod.clone().into_iter());

        let mut result: Vec<Packet64> = builder.collect();
        assert_eq!(result.len(), 1);
        assert_eq!(sequence_number, SequenceNumber::new(2));

        let packet = result.pop().unwrap();
        println!("Packet = {:?}", packet);

        assert!(packet.validate());
        assert_eq!(packet.sequence_number(), SequenceNumber::new(1));
        assert_eq!(packet.kind(), PacketKind::SelfContained);
        assert_eq!(
            packet.payload(),
            u64::from_le_bytes([0x01, 0x02, 0xab, 0xcd, 0x00, 0x00, 0x00, 0x00])
        );
        assert_eq!(packet.payload_used_index(), 5 - 1);
    }

    #[test]
    fn test_complex_packet_builder() {
        // Payload for 3 not complete packet
        let paylaod = vec![
            0x01u8, 0x02, 0x03, 0x04, 0x05, 0xa0u8, 0xb0, 0xc0, 0xd0, 0xe0, 0x11u8, 0x22, 0x33,
            0x44,
        ];
        let mut sequence_number = SequenceNumber::new(1);
        let address = Address::new(0x01, 0x02);
        let builder = PacketBuilder::new(&address, &mut sequence_number, paylaod.into_iter());

        let result: Vec<Packet64> = builder.collect();
        println!("{:?}", result);
        assert_eq!(result.len(), 3);

        assert_eq!(result[0].kind(), PacketKind::Start);
        assert_eq!(result[1].kind(), PacketKind::Continue);
        assert_eq!(result[2].kind(), PacketKind::End);

        // TODO add asserts for multiple packets
    }
}
