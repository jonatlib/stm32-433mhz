use crate::{Codec, CodecSize};

use reed_solomon::{Decoder, Encoder};

pub struct ReedSolomon<const ECC_LEN: usize, const ENCODE_BUFFER_SIZE: usize> {
    encoder: Encoder,
    decoder: Decoder,
}

impl<const ECC_LEN: usize, const ENCODE_BUFFER_SIZE: usize>
    ReedSolomon<ECC_LEN, ENCODE_BUFFER_SIZE>
{
    const DECODE_BUFFER_SIZE: usize = ENCODE_BUFFER_SIZE + ECC_LEN;
}

impl<const ECC_LEN: usize, const ENCODE_BUFFER_SIZE: usize> Default
    for ReedSolomon<ECC_LEN, ENCODE_BUFFER_SIZE>
{
    fn default() -> Self {
        Self {
            encoder: Encoder::new(ECC_LEN),
            decoder: Decoder::new(ECC_LEN),
        }
    }
}

impl<const ECC_LEN: usize, const ENCODE_BUFFER_SIZE: usize> Codec
    for ReedSolomon<ECC_LEN, ENCODE_BUFFER_SIZE>
where
    [(); Self::DECODE_BUFFER_SIZE]: Sized,
    [(); Self::get_encode_const_size(ENCODE_BUFFER_SIZE)]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        let encoded: heapless::Vec<_, { Self::get_encode_const_size(ENCODE_BUFFER_SIZE) }> =
            self.encoder.encode(payload).into_iter().copied().collect();

        encoded.into_iter()
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        //FIXME decode (maybe even encode) can return an error
        let decode_buffer = self.decoder.correct(payload, None).expect("TODO");
        let mut decoded: heapless::Vec<_, { Self::DECODE_BUFFER_SIZE }> =
            decode_buffer.into_iter().copied().collect();
        decoded
            .resize(decoded.len() - ECC_LEN, 0)
            .expect("This should not fail");
        decoded.into_iter()
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size + ECC_LEN
    }
}

impl<const ECC_LEN: usize, const ENCODE_BUFFER_SIZE: usize> const CodecSize
    for ReedSolomon<ECC_LEN, ENCODE_BUFFER_SIZE>
{
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size + ECC_LEN
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_encode_decode() {
        let codec = ReedSolomon::<4, 4>::default();
        let payload = vec![1u8, 2, 3];

        let encoded: Vec<_> = codec.encode(&payload[..]).collect();
        assert_ne!(encoded, payload);

        let decoded: Vec<_> = codec.decode(&encoded[..]).collect();
        assert_eq!(payload, decoded);
    }
}
