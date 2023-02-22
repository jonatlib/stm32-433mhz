use crate::{Codec, CodecError, CodecSize};

use heapless::Vec;

#[derive(Default)]
pub struct Chain<CodecA: Default, CodecB: Default, const INPUT_DATA_SIZE: usize> {
    codec_a: CodecA,
    codec_b: CodecB,
}

pub type Chain2<A, B, const INPUT_DATA_SIZE: usize> = Chain<A, B, INPUT_DATA_SIZE>;

#[allow(type_alias_bounds)] // Used for const generic computation
pub type Chain3<A, B, C, const INPUT_DATA_SIZE: usize>
where
    A: Default + Codec + ~const CodecSize,
    B: Default + Codec + ~const CodecSize,
    C: Default + Codec + ~const CodecSize,
= Chain<A, Chain<B, C, { max_size_1::<A>(INPUT_DATA_SIZE) }>, INPUT_DATA_SIZE>;

pub const fn max_size_1<CodecA: ~const CodecSize>(input: usize) -> usize {
    CodecA::get_encode_const_size(input)
}

pub const fn max_size_2<CodecA: ~const CodecSize, CodecB: ~const CodecSize>(input: usize) -> usize {
    CodecB::get_encode_const_size(CodecA::get_encode_const_size(input))
}

impl<CodecA, CodecB, const INPUT_DATA_SIZE: usize> Codec for Chain<CodecA, CodecB, INPUT_DATA_SIZE>
where
    CodecA: Default + Codec + ~const CodecSize,
    CodecB: Default + Codec + ~const CodecSize,

    [(); max_size_1::<CodecA>(INPUT_DATA_SIZE)]: Sized,
    // [(); max_size_2::<CodecA, CodecB>(INPUT_DATA_SIZE)]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        let a_encoded: Vec<_, { max_size_1::<CodecA>(INPUT_DATA_SIZE) }> =
            self.codec_a.encode(payload)?.collect();

        // TODO some weird error with lifetime when we use const generics here. WTF?
        // let b_encoded: Vec<_, { max_size_2::<CodecA, CodecB>(INPUT_DATA_SIZE) }> =
        //     self.codec_b.encode(&a_encoded[..])?.collect();
        let b_encoded: Vec<_, 128> = self.codec_b.encode(&a_encoded[..])?.collect();

        Ok(b_encoded.into_iter())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        let b_decoded: Vec<_, { max_size_1::<CodecA>(INPUT_DATA_SIZE) }> =
            self.codec_b.decode(payload)?.collect();
        let a_decoded: Vec<_, { INPUT_DATA_SIZE }> = self.codec_a.decode(&b_decoded[..])?.collect();

        Ok(a_decoded.into_iter())
    }

    fn get_encode_size(payload_size: usize) -> usize {
        debug_assert!(payload_size <= INPUT_DATA_SIZE);
        CodecB::get_encode_size(CodecA::get_encode_size(payload_size))
    }
}

impl<CodecA: Default, CodecB: Default, const INPUT_DATA_SIZE: usize> const CodecSize
    for Chain<CodecA, CodecB, INPUT_DATA_SIZE>
where
    CodecA: ~const CodecSize,
    CodecB: ~const CodecSize,
{
    fn get_encode_const_size(payload_size: usize) -> usize {
        debug_assert!(payload_size <= INPUT_DATA_SIZE);
        CodecB::get_encode_const_size(CodecA::get_encode_const_size(payload_size))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lzss::LzssCompression;
    use crate::reed_solomon::ReedSolomon;
    use crate::Identity;
    use std::vec::Vec;

    #[test]
    fn test_encode_decode_chain_2() {
        let codec = Chain2::<Identity, Identity, 4>::default();
        let payload = vec![1u8, 2, 3];

        let encoded: Vec<_> = codec
            .encode(&payload[..])
            .expect("There should be no error")
            .collect();

        let decoded: Vec<_> = codec
            .decode(&encoded[..])
            .expect("There should be no error")
            .collect();
        assert_eq!(payload, decoded);
    }

    #[test]
    fn test_encode_decode_chain_3() {
        let codec = Chain3::<Identity, Identity, Identity, 4>::default();
        let payload = vec![1u8, 2, 3];

        let encoded: Vec<_> = codec
            .encode(&payload[..])
            .expect("There should be no error")
            .collect();

        let decoded: Vec<_> = codec
            .decode(&encoded[..])
            .expect("There should be no error")
            .collect();
        assert_eq!(payload, decoded);
    }

    #[test]
    fn test_encode_decode_chain_3_non_identity() {
        let codec = Chain3::<Identity, ReedSolomon<4, 4>, Identity, 4>::default();
        let payload = vec![1u8, 2, 3];

        let encoded: Vec<_> = codec
            .encode(&payload[..])
            .expect("There should be no error")
            .collect();
        assert_ne!(encoded, payload);

        let decoded: Vec<_> = codec
            .decode(&encoded[..])
            .expect("There should be no error")
            .collect();
        assert_eq!(payload, decoded);
    }

    #[test]
    fn test_encode_decode_chain_3_non_identity_complex() {
        let codec = Chain3::<ReedSolomon<4, 4>, LzssCompression, ReedSolomon<4, 8>, 4>::default();
        let payload = vec![1u8, 2, 3];

        let encoded: Vec<_> = codec
            .encode(&payload[..])
            .expect("There should be no error")
            .collect();
        assert_ne!(encoded, payload);

        let decoded: Vec<_> = codec
            .decode(&encoded[..])
            .expect("There should be no error")
            .collect();
        assert_eq!(payload, decoded);
    }
}
