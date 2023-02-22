use crate::{Codec, CodecError, CodecSize};

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

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        let encoded: heapless::Vec<_, { Self::get_encode_const_size(ENCODE_BUFFER_SIZE) }> =
            self.encoder.encode(payload).iter().copied().collect();

        Ok(encoded.into_iter())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        let decode_buffer = self
            .decoder
            .correct(payload, None)
            .map_err(|_| CodecError::DecodeError)?;
        let mut decoded: heapless::Vec<_, { Self::DECODE_BUFFER_SIZE }> =
            decode_buffer.iter().copied().collect();
        decoded
            .resize(decoded.len() - ECC_LEN, 0)
            .expect("This should not fail");
        Ok(decoded.into_iter())
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
    fn test_bit_flip() {
        let codec = ReedSolomon::<4, 4>::default();
        let payload = vec![1u8, 2, 3, 4];

        let mut encoded: Vec<_> = codec
            .encode(&payload[..])
            .expect("There should be no error")
            .collect();
        assert_ne!(encoded, payload);

        encoded[0] = 3;
        encoded[3] = 3;

        let decoded: Vec<_> = codec
            .decode(&encoded[..])
            .expect("There should be no error")
            .collect();
        assert_eq!(payload, decoded);
    }
}
