use embassy_time::Duration;

pub struct ManchesterTiming {
    pub encoding_between_half_bits: Duration,
    pub decoding_start_wait: Duration,
    pub decoding_middle_wait: Duration,
    pub decoding_end_wait: Duration,
}

pub fn create_manchester_timing(data_timing: Duration) -> ManchesterTiming {
    let half = data_timing / 2;
    let quarter = half / 2;

    ManchesterTiming {
        encoding_between_half_bits: half,

        decoding_start_wait: quarter,
        decoding_middle_wait: half,
        decoding_end_wait: quarter,
    }
}

pub enum BitOrder {
    /// LSB
    LittleEndian,
    /// MSB
    BigEndian,
}

pub struct EncoderIterator<I> {
    bit_order: BitOrder,
    data: I,

    current_byte: Option<u8>,
    current_index: u8,

    pub bytes_encoded: usize,
}

impl<I: Iterator<Item = u8>> EncoderIterator<I> {
    pub fn new(data: I, bit_order: BitOrder) -> Self {
        Self {
            bit_order,
            data,
            current_byte: None,
            current_index: 0,
            bytes_encoded: 0,
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for EncoderIterator<I> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= 8 {
            self.current_byte = None;
            self.current_index = 0;
            self.bytes_encoded += 1;
        }

        let current_byte = self.current_byte.or(self.data.next())?;
        if self.current_byte.is_none() {
            self.current_byte = Some(current_byte);
            self.current_index = 0;
        }

        let result = match self.bit_order {
            BitOrder::LittleEndian => (current_byte >> self.current_index) & 0x01,
            BitOrder::BigEndian => todo!("Bit endian in manchester not implemented"),
        };

        self.current_index += 1;
        Some(result > 0)
    }
}
