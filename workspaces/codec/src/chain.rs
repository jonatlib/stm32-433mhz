use crate::{Codec, CodecError, CodecSize};

#[derive(Default)]
pub struct Chain<CodecA: Default, CodecB: Default> {
    codec_a: CodecA,
    codec_b: CodecB,
}

impl<CodecA: Default, CodecB: Default> Codec for Chain<CodecA, CodecB> {
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

impl<CodecA: Default, CodecB: Default> const CodecSize for Chain<CodecA, CodecB> {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size
    }
}
