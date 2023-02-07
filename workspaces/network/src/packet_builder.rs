use crate::packet::{Packet32, PacketKind};
use crate::Address;

pub struct PacketBuilder<'a, I> {
    address: &'a Address,

    start_sequence_number: u8,
    sequence_number_index: u8,

    first_element: bool,
    self_contained: bool,
    prev_element: Option<&'a u8>,

    payload: I,
}

impl<'a, I> PacketBuilder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    pub fn new(address: &'a Address, start_sequence_number: u8, mut payload: I) -> Self {
        Self {
            address,
            start_sequence_number,
            sequence_number_index: 0,
            first_element: true,
            self_contained: true,
            prev_element: payload.next(),
            payload,
        }
    }
}

impl<'a, I> Iterator for PacketBuilder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = Packet32;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_0 = *self.prev_element?;
        let byte_1 = self.payload.next().copied();
        let byte_2 = self.payload.next();

        self.prev_element = byte_2;
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

        // TODO some modulo
        // FIXME add sequence number counter for window also
        let sequence_number = self.start_sequence_number + self.sequence_number_index;
        // FIXME this should be in the struct like calling `advance`
        self.sequence_number_index += 1;

        let payload =
            u16::from_be_bytes([byte_0, if let Some(byte) = byte_1 { byte } else { 0x00 }]);

        Some(
            Packet32::new()
                .with_kind(kind)
                .with_sequence_number(sequence_number)
                .with_source_address(self.address.local_address)
                .with_destination_address(self.address.destination_address)
                .with_payload(payload)
                .with_both_bytes_used(byte_1.is_some()),
        )
    }
}
