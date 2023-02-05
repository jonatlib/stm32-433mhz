use super::BytesCodec;

use itertools::Itertools;

pub struct SixBitsCodec {}

impl Default for SixBitsCodec {
    fn default() -> Self {
        return Self {};
    }
}

impl<Size> BytesCodec<heapless::Vec<u8, Size>> for SixBitsCodec
where
    Size: heapless::ArrayLength<u8>,
{
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> heapless::Vec<u8, Size> {
        const SYMBOLS: [u8; 16] = [
            0xd, 0xe, 0x13, 0x15, 0x16, 0x19, 0x1a, 0x1c, 0x23, 0x25, 0x26, 0x29, 0x2a, 0x2c, 0x32,
            0x34,
        ];

        let mut finished = true;
        let mut prev_value: u32 = 0;
        let mut prev_bits_used = 0u8;

        let mut data = input
            .into_iter()
            .map(|v: u8| {
                ((SYMBOLS[(v >> 4) as usize] as u16) << 6) | SYMBOLS[(v & 0xf) as usize] as u16
            })
            .fold(heapless::Vec::<u8, Size>::new(), |mut acc, v: u16| {
                let mut value: u32 = v as u32;
                let mut bits_used = 12u8;

                if !finished {
                    value = (value << prev_bits_used) | prev_value;
                    bits_used = bits_used + prev_bits_used;
                    finished = true;
                }

                // println!("{:#034b} {}", value, bits_used);
                while bits_used >= 8 {
                    let v = (value & 0xff) as u8;
                    acc.push(v).unwrap();

                    value = value >> 8;
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
            });

        if !finished {
            data.push((prev_value & 0xff) as u8).unwrap();
        }

        return data;
    }

    fn decode(&self, input: impl IntoIterator<Item = u8>) -> heapless::Vec<u8, Size> {
        const SYMBOLS: [u8; 16] = [
            0xd, 0xe, 0x13, 0x15, 0x16, 0x19, 0x1a, 0x1c, 0x23, 0x25, 0x26, 0x29, 0x2a, 0x2c, 0x32,
            0x34,
        ];

        let mut finished = true;
        let mut prev_value: u16 = 0;
        let mut prev_bits_used = 0u8;

        input
            .into_iter()
            .fold(heapless::Vec::<u8, Size>::new(), |mut acc, v: u8| {
                let mut value = v as u16;
                let mut bits_used = 8u8;

                if !finished {
                    value = (value << prev_bits_used) | prev_value;
                    bits_used = bits_used + prev_bits_used;
                    finished = true;
                }

                // println!("{:#034b} {}", value, bits_used);
                while bits_used >= 6 {
                    let v = (value & 0x3f) as u8;
                    acc.push(v).unwrap();

                    value = value >> 6;
                    bits_used -= 6;
                    // println!("{:#034b} {}", value, bits_used);
                }

                if bits_used > 0 {
                    finished = false;
                    prev_bits_used = bits_used;
                    prev_value = value;
                }

                acc
            })
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
            })
            .collect()
    }
}
