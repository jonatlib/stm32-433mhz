use crate::{Codec, CodecError, CodecSize};

type BaseCompression<const EI: usize, const EJ: usize> =
    lzss::Lzss<EI, EJ, 0x20, { 1 << EI }, { 2 << EI }>;

const EI: usize = 6;
#[allow(dead_code)] // FIXME once the fixme with const is resolved remove this
const COMPRESSION_BUFFER_SIZE: usize = 2 << EI;
#[allow(dead_code)] // FIXME once the fixme with const is resolved remove this
const DECOMPRESSION_BUFFER_SIZE: usize = 1 << EI;
type Compression = BaseCompression<EI, 3>;

#[derive(Default)]
pub struct LzssCompression {}

impl Codec for LzssCompression {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        let mut compressed = [0u8; { COMPRESSION_BUFFER_SIZE }];
        let compression_result = Compression::compress_stack(
            lzss::SliceReader::new(payload),
            lzss::SliceWriter::new(&mut compressed[..]),
        )
        .map_err(|_| CodecError::EncodeError)?;

        // FIXME dont know why that const generic is not resolved.
        // let output: heapless::Vec<_, COMPRESSION_BUFFER_SIZE> =
        let output: heapless::Vec<_, 128> =
            heapless::Vec::from_iter(compressed[..compression_result].iter().copied());

        Ok(output.into_iter())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        let mut decompressed = [0u8; { DECOMPRESSION_BUFFER_SIZE }];
        let decompression_result = Compression::decompress_stack(
            lzss::SliceReader::new(payload),
            lzss::SliceWriter::new(&mut decompressed[..]),
        )
        .map_err(|_| CodecError::DecodeError)?;

        // FIXME dont know why that const generic is not resolved.
        // let mut decompressed: heapless::Vec<_, { DECOMPRESSION_BUFFER_SIZE }> = heapless::Vec::new();
        let output: heapless::Vec<_, 64> =
            heapless::Vec::from_iter(decompressed[..decompression_result].iter().copied());

        Ok(output.into_iter())
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size
    }
}

impl const CodecSize for LzssCompression {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_compression_type() {
        let payload = vec![1u8, 1, 1, 1, 1, 1, 7, 8, 9, 10];
        let mut compressed = [0u8; 20];
        let mut decompressed = [0u8; 20];

        let compression_result = Compression::compress_stack(
            lzss::SliceReader::new(&payload[..]),
            lzss::SliceWriter::new(&mut compressed),
        );
        // println!("{:?}", compression_result);
        assert!(compression_result.is_ok());
        let wrote_bytes = compression_result.expect("This cant be error");
        assert_ne!(&compressed[..wrote_bytes], &payload[..]);
        assert!(wrote_bytes < payload.len());

        // println!("{}; {:?}", payload.len(), payload);
        // println!("{}; {:?}", wrote_bytes, compressed);
        // println!("{}; {:?}", wrote_bytes, &compressed[..wrote_bytes]);

        let decompression_result = Compression::decompress_stack(
            lzss::SliceReader::new(&compressed[..wrote_bytes]),
            lzss::SliceWriter::new(&mut decompressed),
        );
        // println!("{:?}", decompression_result);
        assert!(decompression_result.is_ok());
        let read_bytes = decompression_result.expect("This cant be error");
        assert_eq!(&decompressed[..read_bytes], &payload[..]);
    }

    #[test]
    fn test_compression_same_as_codec() {
        let payload = vec![1u8, 2, 3];
        let mut compressed = [0u8; { DECOMPRESSION_BUFFER_SIZE }];

        let compression_result = Compression::compress_stack(
            lzss::SliceReader::new(&payload[..]),
            lzss::SliceWriter::new(&mut compressed),
        );

        assert!(compression_result.is_ok())
    }

    #[test]
    fn test_encode_decode() {
        let codec = LzssCompression::default();
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
