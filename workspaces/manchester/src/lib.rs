#![no_std]

// Enable testing on local machine
#[cfg(test)]
#[macro_use]
extern crate std;

use defmt::trace;
use embassy_time::Duration;

#[derive(Debug)]
pub struct ManchesterTiming {
    pub encoding_between_half_bits: Duration,
    pub decoding_start_wait: Duration,
    pub decoding_middle_wait: Duration,
    pub decoding_end_wait: Duration,
    pub decoding_timeout: Duration,
}

pub fn create_manchester_timing(data_timing: Duration) -> ManchesterTiming {
    let half = data_timing / 2;
    let quarter = half / 2;

    ManchesterTiming {
        encoding_between_half_bits: half,

        decoding_start_wait: quarter,
        decoding_middle_wait: half,
        decoding_end_wait: quarter,

        decoding_timeout: data_timing * 15,
    }
}

pub enum BitOrder {
    /// LSB
    LittleEndian,
    /// MSB
    BigEndian,
}

pub struct EncoderBoolIterator<I> {
    bit_order: BitOrder,
    data: I,

    current_byte: Option<u8>,
    current_index: u8,
    next_bit: Option<bool>,
}

impl<I: Iterator<Item = u8>> EncoderBoolIterator<I> {
    pub fn new(data: I, bit_order: BitOrder) -> Self {
        Self {
            bit_order,
            data,
            current_byte: None,
            current_index: 0,
            next_bit: None,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for EncoderBoolIterator<I> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_bit) = self.next_bit {
            self.next_bit = None;
            return Some(next_bit);
        }

        if self.current_index >= 8 {
            self.current_byte = None;
            self.current_index = 0;
            self.next_bit = None;
        }

        let current_byte = self.current_byte.or_else(|| self.data.next())?;
        if self.current_byte.is_none() {
            self.current_byte = Some(current_byte);
            self.current_index = 0;
            self.next_bit = None;
        }

        let result = match self.bit_order {
            BitOrder::LittleEndian => (current_byte >> self.current_index) & 0x01,
            BitOrder::BigEndian => todo!("Bit endian in manchester not implemented"),
        };

        self.current_index += 1;
        if result > 0 {
            self.next_bit = Some(true);
            return Some(false);
        } else {
            self.next_bit = Some(false);
            return Some(true);
        }
    }
}

pub struct DecoderBool {
    bit_order: BitOrder,

    pair_0: Option<bool>,
    pair_1: Option<bool>,

    current_byte: u8,
    current_index: u8,
}

impl DecoderBool {
    pub fn new(bit_order: BitOrder) -> Self {
        Self {
            bit_order,

            pair_0: None,
            pair_1: None,

            current_byte: 0,
            current_index: 0,
        }
    }

    pub fn next(&mut self, input: bool) -> Option<u8> {
        // trace!(
        //     "Manchester decoder[state: {:b}; index: {}], received = {}, pair = ({:?}, {:?})",
        //     self.current_byte,
        //     self.current_index,
        //     input,
        //     self.pair_0,
        //     self.pair_1,
        // );
        if self.pair_0.is_none() {
            self.pair_0 = Some(input);
            return None;
        }

        if self.pair_1.is_none() {
            self.pair_1 = Some(input);
        }

        let pair = (self.pair_0.unwrap(), self.pair_1.unwrap());
        self.pair_0 = None;
        self.pair_1 = None;

        // TODO wha about the invalid values?
        let received_bit = match pair {
            (false, false) => false, // Just a heuristic
            (false, true) => true,   // Correct value by IEEE802.3
            (true, false) => false,  // Correct value by IEEE802.3
            (true, true) => true,    // Just a heuristic
        };

        // FIXME implement other bit orders
        if received_bit {
            self.current_byte |= (0x01 << self.current_index);
        }
        self.current_index += 1;

        if self.current_index >= 8 {
            let result = self.current_byte;

            self.pair_0 = None;
            self.pair_1 = None;
            self.current_byte = 0;
            self.current_index = 0;

            return Some(result);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::vec::Vec;

    #[test]
    fn test_encode_bits() {
        let payload = [0x01; 1];
        let mut encoder = EncoderBoolIterator::new(payload.iter().copied(), BitOrder::LittleEndian);
        let result: Vec<bool> = encoder.collect();

        assert_eq!(
            vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,]
                .into_iter()
                .map(|v| v > 0)
                .collect::<Vec<bool>>(),
            result
        );
    }

    #[test]
    fn test_decode_bits() {
        let payload = vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]
            .into_iter()
            .map(|v| v > 0)
            .collect::<Vec<bool>>();
        let mut decoder = DecoderBool::new(BitOrder::LittleEndian);
        let mut result: Vec<u8> = Vec::new();

        for bit in payload {
            if let Some(byte) = decoder.next(bit) {
                result.push(byte);
            }
        }

        assert_eq!(result, [0x01; 1]);
    }

    #[test]
    fn test_encode_decode() {
        let payload = [1u8, 2, 3, 4, 5];
        let mut encoder = EncoderBoolIterator::new(payload.iter().copied(), BitOrder::LittleEndian);
        let mut decoder = DecoderBool::new(BitOrder::LittleEndian);

        let mut result: Vec<u8> = Vec::new();
        for bit in encoder {
            if let Some(byte) = decoder.next(bit) {
                result.push(byte);
            }
        }

        assert_eq!(Vec::from(payload), result);
    }
}
