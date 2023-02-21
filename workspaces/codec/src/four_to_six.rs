use crate::{Codec, CodecError, CodecSize};

#[derive(Default)]
pub struct FourToSixBits {}

impl Codec for FourToSixBits {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        Ok(payload.iter().copied())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        Ok(payload.iter().copied())
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size
    }
}

impl const CodecSize for FourToSixBits {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size
    }
}
