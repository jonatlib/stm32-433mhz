use core::iter::Iterator;
use itertools::Itertools;

use crate::{Codec, CodecError, CodecSize};

#[derive(Default)]
pub struct FourToSixBits<const MAX_INPUT_SIZE: usize> {}

impl<const MAX_INPUT_SIZE: usize> Codec for FourToSixBits<MAX_INPUT_SIZE>
where
    [(); Self::get_encode_const_size(MAX_INPUT_SIZE)]: Sized,
{
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        const SYMBOLS: [u8; 16] = [
            0xd, 0xe, 0x13, 0x15, 0x16, 0x19, 0x1a, 0x1c, 0x23, 0x25, 0x26, 0x29, 0x2a, 0x2c, 0x32,
            0x34,
        ];

        let mut finished = true;
        let mut prev_value: u32 = 0;
        let mut prev_bits_used = 0u8;

        let mut data = payload
            .iter()
            .copied()
            .map(|v: u8| {
                ((SYMBOLS[(v >> 4) as usize] as u16) << 6) | SYMBOLS[(v & 0xf) as usize] as u16
            })
            .fold(
                heapless::Vec::<u8, { Self::get_encode_const_size(MAX_INPUT_SIZE) }>::new(),
                |mut acc, v: u16| {
                    let mut value: u32 = v as u32;
                    let mut bits_used = 12u8;

                    if !finished {
                        value = (value << prev_bits_used) | prev_value;
                        bits_used += prev_bits_used;
                        finished = true;
                    }

                    // println!("{:#034b} {}", value, bits_used);
                    while bits_used >= 8 {
                        let v = (value & 0xff) as u8;
                        acc.push(v).unwrap();

                        value >>= 8;
                        bits_used -= 8;

                        // println!("{:#034b} {}", value, bits_used);
                    }

                    if bits_used > 0 {
                        // println!("Not finished");
                        finished = false;
                        prev_bits_used = bits_used;
                        prev_value = value;
                    }

                    acc
                },
            );

        if !finished {
            data.push((prev_value & 0xff) as u8).unwrap();
        }

        Ok(data.into_iter())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        const SYMBOLS: [u8; 16] = [
            0xd, 0xe, 0x13, 0x15, 0x16, 0x19, 0x1a, 0x1c, 0x23, 0x25, 0x26, 0x29, 0x2a, 0x2c, 0x32,
            0x34,
        ];

        let mut finished = true;
        let mut prev_value: u16 = 0;
        let mut prev_bits_used = 0u8;

        let result = payload
            .iter()
            .copied()
            .fold(
                heapless::Vec::<u8, { Self::get_encode_const_size(MAX_INPUT_SIZE) }>::new(),
                |mut acc, v: u8| {
                    let mut value = v as u16;
                    let mut bits_used = 8u8;

                    if !finished {
                        value = (value << prev_bits_used) | prev_value;
                        bits_used += prev_bits_used;
                        finished = true;
                    }

                    // println!("{:#034b} {}", value, bits_used);
                    while bits_used >= 6 {
                        let v = (value & 0x3f) as u8;
                        acc.push(v).unwrap();

                        value >>= 6;
                        bits_used -= 6;
                        // println!("{:#034b} {}", value, bits_used);
                    }

                    if bits_used > 0 {
                        finished = false;
                        prev_bits_used = bits_used;
                        prev_value = value;
                    }

                    acc
                },
            )
            .into_iter()
            .tuples::<(u8, u8)>()
            .map(|(i1, i2): (u8, u8)| {
                let a = SYMBOLS.iter().position(|&s| s == i1).unwrap_or_else(|| {
                    SYMBOLS
                        .iter()
                        .enumerate()
                        .map(|(pos, &value)| (pos, (value ^ i1).count_ones()))
                        .min_by(|a, b| a.1.cmp(&b.1))
                        .take()
                        .unwrap()
                        .0
                }) as u8;
                let b = SYMBOLS.iter().position(|&s| s == i2).unwrap_or_else(|| {
                    SYMBOLS
                        .iter()
                        .enumerate()
                        .map(|(pos, &value)| (pos, (value ^ i2).count_ones()))
                        .min_by(|a, b| a.1.cmp(&b.1))
                        .take()
                        .unwrap()
                        .0
                }) as u8;

                (b << 4) | (a & 0xf)
            });

        Ok(result)
    }

    fn get_encode_size(payload_size: usize) -> usize {
        Self::get_encode_const_size(payload_size)
    }
}

impl<const MAX_INPUT_SIZE: usize> const CodecSize for FourToSixBits<MAX_INPUT_SIZE> {
    fn get_encode_const_size(payload_size: usize) -> usize {
        debug_assert!(payload_size <= MAX_INPUT_SIZE);

        (6 * ((payload_size * 8) / 4)).div_ceil(8) + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn test_encode_decode() {
        let codec = FourToSixBits::<3>::default();
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
