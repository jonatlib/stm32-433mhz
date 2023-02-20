use crate::{Codec, CodecSize};

use reed_solomon::{Decoder, Encoder};

#[derive(Default)]
pub struct ReedSolomon<const ECC_LEN: usize> {}

impl<const ECC_LEN: usize> Codec for ReedSolomon<ECC_LEN>
where
    [(); ReedSolomon::<{ ECC_LEN }>::get_encode_const_size(4)]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        let encoder = Encoder::new(ECC_LEN);
        let encoded: heapless::Vec<_, { ReedSolomon::<{ ECC_LEN }>::get_encode_const_size(4) }> =
            encoder.encode(payload).into_iter().copied().collect();

        encoded.into_iter()
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        payload.iter().copied()
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size + ECC_LEN //FIXME is this correct?
    }
}

impl<const ECC_LEN: usize> const CodecSize for ReedSolomon<ECC_LEN> {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size + ECC_LEN //FIXME is this correct?
    }
}
