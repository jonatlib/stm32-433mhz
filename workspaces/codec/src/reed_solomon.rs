use crate::{Codec, CodecSize};

#[derive(Default)]
pub struct ReedSolomon {}

impl Codec for ReedSolomon {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        payload.iter().copied()
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        payload.iter().copied()
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size
    }
}

impl const CodecSize for ReedSolomon {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size
    }
}
