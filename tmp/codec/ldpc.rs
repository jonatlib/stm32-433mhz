use super::{BytesCodec, ChainedData};

use core::iter::FromIterator;

use heapless;
use labrador_ldpc::LDPCCode;

pub struct Ldpc {}

impl Default for Ldpc {
    fn default() -> Self {
        return Self {};
    }
}

impl BytesCodec<ChainedData> for Ldpc {
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let code = LDPCCode::TC256;
        let input_data: heapless::Vec<u8, heapless::consts::U32> = heapless::Vec::from_iter(input);
        let mut result_data: heapless::Vec<u8, heapless::consts::U64> =
            heapless::Vec::from_slice(&[0u8; 32][..]).unwrap();

        let len = input_data.len();
        let mut codec_input: heapless::Vec<u8, heapless::consts::U32> = heapless::Vec::new();

        codec_input.push(len as u8).unwrap();
        codec_input.extend(input_data);

        while codec_input.len() < 16 {
            codec_input.push(0).unwrap();
        }

        code.copy_encode(&codec_input[..], &mut result_data);

        return result_data;
    }

    fn decode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let code = LDPCCode::TC256;

        let mut input_data: heapless::Vec<u8, heapless::consts::U32> =
            heapless::Vec::from_iter(input);
        let mut result_data: heapless::Vec<u8, heapless::consts::U32> =
            heapless::Vec::from_slice(&[0u8; 32][..]).unwrap();
        let mut working_area: heapless::Vec<u8, heapless::consts::U256> =
            heapless::Vec::from_slice(&[0u8; 256][..]).unwrap();

        while input_data.len() < 32 {
            input_data.push(0).unwrap();
        }

        code.decode_bf(&input_data[..], &mut result_data, &mut working_area, 40);
        let len = std::cmp::min(*&result_data[0], 32u8);
        return heapless::Vec::from_slice(
            &result_data[1..std::cmp::min(len + 1, result_data.len() as u8) as usize],
        )
        .unwrap();
    }
}
