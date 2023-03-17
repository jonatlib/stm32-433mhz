use defmt::trace;

use futures::stream::Stream;

use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Pin;
use embassy_time::{with_timeout, Duration, Instant};

use crate::error::ReadError;
use crate::pwm::sync::SyncSequence;
use crate::pwm::writer::WriterTiming;

pub struct ReaderTiming {
    pub zeroes: Duration,
    pub ones: Duration,
    pub lower_threshold: Duration,
    pub upper_threshold: Duration,
}

impl ReaderTiming {
    pub fn new(
        zeroes: Duration,
        ones: Duration,
        lower_threshold: Duration,
        upper_threshold: Duration,
    ) -> Self {
        Self {
            zeroes,
            ones,
            lower_threshold,
            upper_threshold,
        }
    }

    pub fn adjust_to_sync_marker(&mut self, marker: &SyncSequence) {
        self.upper_threshold = marker.ones + marker.zeroes
    }
}

impl Default for ReaderTiming {
    fn default() -> Self {
        Self::new(
            Duration::from_micros(1500),
            Duration::from_micros(5500),
            Duration::from_millis(1),
            Duration::from_millis(15),
        )
    }
}

impl From<&WriterTiming> for ReaderTiming {
    fn from(value: &WriterTiming) -> Self {
        Self::new(
            value.zeroes - Duration::from_micros(500),
            value.ones - Duration::from_micros(500),
            value.zeroes - Duration::from_micros(1000),
            value.ones + value.zeroes,
        )
    }
}

pub trait PwmReader: crate::BaseReader {
    async fn read_timing(&mut self) -> Result<Duration, ReadError>;
    fn get_timing(&self) -> &ReaderTiming;
    fn get_mut_timing(&mut self) -> &mut ReaderTiming;

    #[inline]
    async fn read_bit(&mut self) -> Result<bool, ReadError> {
        let elapsed = self.read_timing().await?;
        if elapsed <= self.get_timing().lower_threshold {
            return Err(ReadError::ThresholdError);
        }

        if elapsed >= self.get_timing().upper_threshold {
            return Err(ReadError::ThresholdError);
        }

        if elapsed >= self.get_timing().ones {
            return Ok(true);
        }

        if elapsed >= self.get_timing().zeroes {
            return Ok(false);
        }

        Err(ReadError::OutOfTiming)
    }

    async fn read_bytes(&mut self, count: usize, buffer: &mut [u8]) -> Result<usize, ReadError> {
        let mut index = 0usize;
        while index < count {
            match self.read_byte().await {
                Ok(byte) => {
                    buffer[index] = byte;
                    // trace!("Received byte = {:#04x} on index = {}", byte, index);
                    index += 1;
                }
                Err(e) => {
                    trace!("Could not read whole byte on index = {}", index);
                    return Err(e);
                }
            }
        }
        Ok(index)
    }

    async fn read_byte(&mut self) -> Result<u8, ReadError> {
        // FIXME how resolve when to return None? how many bit waits?
        let mut index = 0u8;
        let mut value = 0u8;

        loop {
            match self.read_bit().await {
                Ok(bit) => {
                    let bit_value: u8 = (bit as u8) << index;
                    value |= bit_value;
                    // trace!(
                    //     "Constructing byte = {:#010b}; index = {}; read = {}",
                    //     value,
                    //     index,
                    //     bit
                    // );

                    index += 1;
                    if index >= 8 {
                        return Ok(value);
                    }
                }
                Err(e) => {
                    if !e.is_recoverable() {
                        return Err(e);
                    }
                }
            }
        }
    }
}

pub struct PinPwmReader<'a, P: Pin, const INVERT: bool = false> {
    timing: ReaderTiming,
    pin: ExtiInput<'a, P>,
}

impl<'a, P: Pin, const INVERT: bool> PinPwmReader<'a, P, INVERT> {
    #[allow(clippy::result_unit_err)]
    pub fn new(timing: ReaderTiming, pin: ExtiInput<'a, P>) -> Result<Self, ()> {
        Ok(Self { timing, pin })
    }

    pub fn into_stream(self) -> impl Stream<Item = u8> + 'a {
        futures::stream::unfold(self, |mut reader| async {
            reader.read_byte().await.ok().map(|v| (v, reader))
        })
    }
}

impl<'a, P: Pin, const INVERT: bool> crate::BaseReader for PinPwmReader<'a, P, INVERT> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        self.read_bytes(buffer.len(), buffer).await
    }
}

impl<'a, P: Pin, const INVERT: bool> PwmReader for PinPwmReader<'a, P, INVERT> {
    #[inline]
    async fn read_timing(&mut self) -> Result<Duration, ReadError> {
        if INVERT {
            self.pin.wait_for_falling_edge().await;
        } else {
            self.pin.wait_for_rising_edge().await;
        }
        let start_time = Instant::now();

        if INVERT {
            with_timeout(self.timing.upper_threshold, self.pin.wait_for_rising_edge())
                .await
                .map_err(|_| ReadError::TimeoutError)?;
        } else {
            with_timeout(
                self.timing.upper_threshold,
                self.pin.wait_for_falling_edge(),
            )
            .await
            .map_err(|_| ReadError::TimeoutError)?;
        }
        Ok(Instant::now() - start_time)
    }

    fn get_timing(&self) -> &ReaderTiming {
        &self.timing
    }

    fn get_mut_timing(&mut self) -> &mut ReaderTiming {
        &mut self.timing
    }
}
