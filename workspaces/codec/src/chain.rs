use crate::{Codec, CodecError, CodecSize};

use core::marker::PhantomData;
use heapless::Vec;

#[derive(Default)]
pub struct Chain<'this, CodecA: Default, CodecB: Default, const INPUT_DATA_SIZE: usize> {
    codec_a: CodecA,
    codec_b: CodecB,

    _phantom: PhantomData<&'this ()>,
}

impl<'this, CodecA, CodecB, const INPUT_DATA_SIZE: usize> Codec
    for Chain<'this, CodecA, CodecB, INPUT_DATA_SIZE>
where
    CodecA: Default + Codec + ~const CodecSize + 'this,
    CodecB: Default + Codec + ~const CodecSize + 'this,

    [(); CodecA::get_encode_const_size(INPUT_DATA_SIZE)]: Sized,
    [(); CodecB::get_encode_const_size(CodecA::get_encode_const_size(INPUT_DATA_SIZE))]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a where 'a: 'this;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a where 'a: 'this;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        let a_encoded: Vec<_, { CodecA::get_encode_const_size(INPUT_DATA_SIZE) }> =
            self.codec_a.encode(payload)?.collect();

        let b_encoded: Vec<
            _,
            { CodecB::get_encode_const_size(CodecA::get_encode_const_size(INPUT_DATA_SIZE)) },
        > = self.codec_b.encode(&a_encoded[..])?.collect();

        Ok(b_encoded.into_iter())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        let data: Vec<_, INPUT_DATA_SIZE> = payload.iter().copied().collect();

        Ok(data.into_iter())
    }

    fn get_encode_size(payload_size: usize) -> usize {
        debug_assert!(payload_size <= INPUT_DATA_SIZE);
        CodecB::get_encode_size(CodecA::get_encode_size(payload_size))
    }
}

impl<CodecA: Default, CodecB: Default, const INPUT_DATA_SIZE: usize> const CodecSize
    for Chain<'_, CodecA, CodecB, INPUT_DATA_SIZE>
where
    CodecA: ~const CodecSize,
    CodecB: ~const CodecSize,
{
    fn get_encode_const_size(payload_size: usize) -> usize {
        debug_assert!(payload_size <= INPUT_DATA_SIZE);
        CodecB::get_encode_const_size(CodecA::get_encode_const_size(payload_size))
    }
}
