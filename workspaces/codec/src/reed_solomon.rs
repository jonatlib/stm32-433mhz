use crate::{Codec, CodecSize};

use reed_solomon::{Decoder, Encoder};

pub struct ReedSolomon<const ECC_LEN: usize> {
    encoder: Encoder,
    decoder: Decoder,
}

impl<const ECC_LEN: usize> Default for ReedSolomon<ECC_LEN> {
    fn default() -> Self {
        Self {
            encoder: Encoder::new(ECC_LEN),
            decoder: Decoder::new(ECC_LEN),
        }
    }
}

impl<const ECC_LEN: usize> Codec for ReedSolomon<ECC_LEN>
where
    [(); ReedSolomon::<{ ECC_LEN }>::get_encode_const_size(4)]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        // let encoded: heapless::Vec<_, { ReedSolomon::<{ ECC_LEN }>::get_encode_const_size(4) }> =
        let encoded: heapless::Vec<_, { Self::get_encode_const_size(4) }> =
            self.encoder.encode(payload).into_iter().copied().collect();

        encoded.into_iter()
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        //FIXME decode (maybe even encode) can return an error
        let decode_buffer = self.decoder.correct(payload, None).expect("TODO");
        let decoded: heapless::Vec<_, 4> = decode_buffer.into_iter().copied().collect();
        decoded.into_iter()
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
