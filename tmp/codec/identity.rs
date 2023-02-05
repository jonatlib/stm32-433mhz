use super::BytesCodec;

use core::iter::FromIterator;

pub struct Identity {}

impl Default for Identity {
    fn default() -> Self {
        return Self {};
    }
}

impl<Size> BytesCodec<heapless::Vec<u8, Size>> for Identity
where
    Size: heapless::ArrayLength<u8>,
{
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> heapless::Vec<u8, Size> {
        return heapless::Vec::from_iter(input);
    }

    fn decode(&self, input: impl IntoIterator<Item = u8>) -> heapless::Vec<u8, Size> {
        return heapless::Vec::from_iter(input);
    }
}
