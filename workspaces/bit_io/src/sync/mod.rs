use embassy_stm32::gpio::Pin;
use embassy_time::{Duration, Timer};

use crate::error::{ReadError, WriterError};
use crate::{PinReader, PinWriter, Reader, Writer};

pub mod sync_reader;
pub mod sync_writer;

#[derive(Clone)]
pub struct SyncSequence {
    pub ones: Duration,
    pub zeroes: Duration,
    pub between_bits: Duration,
    pub read_threshold: Duration,

    number_of_bits: u8,
    sequence: u32,
}

impl SyncSequence {
    pub fn new(
        ones: Duration,
        zeroes: Duration,
        between_bits: Duration,
        read_threshold: Duration,

        number_of_bits: u8,
        sequence: u32,
    ) -> Self {
        Self {
            ones,
            zeroes,
            between_bits,
            read_threshold,
            number_of_bits,
            sequence,
        }
    }

    pub fn new_simple(ones: Duration, number_of_bits: u8, sequence: u32) -> Self {
        Self::new(ones, ones / 2, ones / 4, ones / 6, number_of_bits, sequence)
    }

    pub async fn write_sequence<W: Writer>(&self, writer: &mut W) -> Result<(), WriterError> {
        for index in 0..self.number_of_bits {
            let mask = 1u32 << index;
            let bit = (self.sequence & mask) > 0;

            writer
                .write_timing(if bit { self.ones } else { self.zeroes } + self.read_threshold)
                .await?;

            Timer::after(self.between_bits).await;
        }

        Ok(())
    }

    pub async fn read_sequence<R: Reader>(&self, reader: &mut R) -> Result<(), ReadError> {
        let mut index = 0u8;

        loop {
            let value = reader.read_timing().await;
            let time = match value {
                Ok(time) => time,
                Err(e) => {
                    if e.is_recoverable() {
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            };

            let expected = if ((1u32 << index) & self.sequence) > 0 {
                self.ones
            } else {
                self.zeroes
            };

            if time >= expected {
                index += 1;
                if index >= self.number_of_bits {
                    return Ok(());
                }
            } else {
                index = 0;
            }
        }
    }
}

impl Default for SyncSequence {
    fn default() -> Self {
        Self::new_simple(Duration::from_millis(10), 4, 0b1011)
    }
}
