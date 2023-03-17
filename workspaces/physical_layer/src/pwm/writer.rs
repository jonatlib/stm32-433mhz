use embassy_stm32::gpio::{Output, Pin};
use embassy_time::{Duration, Timer};

use crate::error::WriterError;

pub struct WriterTiming {
    pub zeroes: Duration,
    pub ones: Duration,
    pub between_bits: Duration,
    pub between_bytes: Option<Duration>,
}

impl WriterTiming {
    pub fn new(
        zeroes: Duration,
        ones: Duration,
        between_bits: Duration,
        between_bytes: Option<Duration>,
    ) -> Self {
        Self {
            zeroes,
            ones,
            between_bits,
            between_bytes,
        }
    }
}

impl Default for WriterTiming {
    fn default() -> Self {
        Self::new(
            Duration::from_millis(2),
            Duration::from_millis(6),
            Duration::from_millis(1),
            None,
        )
    }
}

pub trait PwmWriter: crate::BaseWriter {
    async fn write_timing(&mut self, duration: Duration) -> Result<(), WriterError>;
    fn get_timing(&self) -> &WriterTiming;

    #[inline]
    async fn write_bit(&mut self, value: bool) -> Result<(), WriterError> {
        self.write_timing(if value {
            self.get_timing().ones
        } else {
            self.get_timing().zeroes
        })
        .await
    }

    async fn write_byte(&mut self, value: u8) -> Result<(), WriterError> {
        // defmt::trace!("Writing byte = {:#04x}", value);
        for index in 0..8u8 {
            let mask = 1u8 << index;
            let bit = (value & mask) > 0;

            self.write_bit(bit).await?;
            Timer::after(self.get_timing().between_bits).await;
        }

        Ok(())
    }

    async fn write_bytes(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        for byte in buffer {
            self.write_byte(*byte).await?;

            if let Some(between_bytes) = self.get_timing().between_bytes {
                Timer::after(between_bytes).await;
            }
        }

        Ok(buffer.len())
    }
}

pub struct PinPwmWriter<'a, P: Pin, const INVERT: bool = false> {
    timing: WriterTiming,
    pin: Output<'a, P>,
}

impl<'a, P: Pin, const INVERT: bool> PinPwmWriter<'a, P, INVERT> {
    #[allow(clippy::result_unit_err)]
    pub fn new(timing: WriterTiming, mut pin: Output<'a, P>) -> Result<Self, ()> {
        if INVERT {
            pin.set_high();
        } else {
            pin.set_low();
        }

        Ok(Self { timing, pin })
    }
}

impl<'a, P: Pin, const INVERT: bool> crate::BaseWriter for PinPwmWriter<'a, P, INVERT> {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.write_bytes(buffer).await
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        let mut bytes = 0usize;
        for byte in data {
            self.write_byte(byte).await?;
            bytes += 1;

            if let Some(between_bytes) = self.get_timing().between_bytes {
                Timer::after(between_bytes).await;
            }
        }

        // Let some time between streams
        Timer::after(self.get_timing().ones * 4).await;

        Ok(bytes)
    }
}

impl<'a, P: Pin, const INVERT: bool> PwmWriter for PinPwmWriter<'a, P, INVERT> {
    #[inline]
    async fn write_timing(&mut self, duration: Duration) -> Result<(), WriterError> {
        if INVERT {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }

        Timer::after(duration).await;

        if INVERT {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }

        Ok(())
    }

    #[inline]
    fn get_timing(&self) -> &WriterTiming {
        &self.timing
    }
}
