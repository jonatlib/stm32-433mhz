use crate::packet::{Packet32, PacketKind};
use crate::Address;

pub struct PacketBuilder<'a, I> {
    address: &'a Address,

    payload: I,
}

impl<'a, I> PacketBuilder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    pub fn new(address: &'a Address, payload: I) -> Self {
        Self { address, payload }
    }
}

impl<'a, I> Iterator for PacketBuilder<'a, I>
where
    I: Iterator<Item = &'a u8>,
{
    type Item = Packet32;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_0 = *(self.payload.next()?);
        let byte_1 = self.payload.next().map(|v| *v);

        let payload =
            u16::from_be_bytes([byte_0, if let Some(byte) = byte_1 { byte } else { 0x00 }]);

        Some(
            Packet32::new()
                .with_kind(PacketKind::SelfContained) // FIXME this is wrong
                .with_sequence_number(0) // FIXME count sequence numbers (even outside of this builder)
                .with_source_address(self.address.source_address)
                .with_destination_address(self.address.destination_address)
                .with_payload(payload)
                .with_both_bytes_used(byte_1.is_some()),
        )
    }
}
