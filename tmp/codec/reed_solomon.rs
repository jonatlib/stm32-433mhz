use super::{BytesCodec, ChainedData};

use core::iter::FromIterator;

use heapless;
use reed_solomon::{Decoder, Encoder};

pub struct ReedSolomon {}

impl ReedSolomon {
    const ECC_LEN: usize = 18;
}

impl Default for ReedSolomon {
    fn default() -> Self {
        return Self {};
    }
}

impl BytesCodec<ChainedData> for ReedSolomon {
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let encoder = Encoder::new(Self::ECC_LEN);
        let input_data: heapless::Vec<u8, heapless::consts::U32> = heapless::Vec::from_iter(input);

        return heapless::Vec::from_slice(&encoder.encode(&input_data[..])[..]).unwrap();
    }

    fn decode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let decoder = Decoder::new(Self::ECC_LEN);
        let input_data: heapless::Vec<u8, heapless::consts::U32> = heapless::Vec::from_iter(input);

        if input_data.len() > Self::ECC_LEN {
            if let Ok(data) = decoder.correct(&input_data[..], None) {
                return heapless::Vec::from_slice(data.data()).unwrap();
            }
        }
        return heapless::Vec::new();
    }
}
