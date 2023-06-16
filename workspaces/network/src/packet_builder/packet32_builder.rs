use core::borrow::Borrow;

use sequence_number::SequenceNumber;

use crate::packet::{Packet32, PacketKind};
use crate::Address;

pub struct PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8>,
{
    address: &'a Address,

    sequence_number: &'a mut SequenceNumber<8>,

    first_element: bool,
    self_contained: bool,
    prev_element: Option<I>,

    payload: P,
}

impl<'a, P, I> PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8>,
{
    pub fn new(
        address: &'a Address,
        start_sequence_number: &'a mut SequenceNumber<8>,
        mut payload: P,
    ) -> Self {
        Self {
            address,
            sequence_number: start_sequence_number,
            first_element: true,
            self_contained: true,
            prev_element: payload.next(),
            payload,
        }
    }
}

impl<'a, P, I> Iterator for PacketBuilder<'a, P, I>
where
    P: Iterator<Item = I>,
    I: Borrow<u8> + Clone,
{
    type Item = Packet32;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_0 = *self.prev_element.as_ref()?.borrow();
        let byte_1 = self.payload.next().map(|v| *v.borrow());
        let byte_2 = self.payload.next();

        self.prev_element = byte_2.clone();
        if byte_2.is_some() {
            self.self_contained = false;
        }

        let kind = if self.self_contained {
            PacketKind::SelfContained
        } else if self.first_element {
            self.first_element = false;
            PacketKind::Start
        } else if byte_2.is_some() {
            PacketKind::Continue
        } else {
            PacketKind::End
        };

        let payload =
            u16::from_le_bytes([byte_0, if let Some(byte) = byte_1 { byte } else { 0x00 }]);

        Some(
            Packet32::new()
                .with_kind(kind)
                .with_sequence_number(self.sequence_number.advance())
                .with_source_address(self.address.local_address)
                .with_destination_address(self.address.destination_address)
                .with_payload(payload)
                .with_payload_used_index(byte_1.is_some() as u8),
        )
    }
}
