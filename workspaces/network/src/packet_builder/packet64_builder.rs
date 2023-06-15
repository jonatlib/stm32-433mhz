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
        // let packet = Packet64::default();

        todo!()
    }
}
