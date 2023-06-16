use crate::error::ReadError;
use crate::utils::SharedPin;
use crate::BaseReader;
use defmt::{debug, trace};
use embassy_stm32::exti::ExtiInput;
use manchester::{create_manchester_timing, BitOrder, DecoderBool, ManchesterTiming};

use embassy_stm32::gpio::{Input, Pin};
use embassy_time::{with_timeout, Duration, Timer};

pub struct ManchesterReader<'a, P: Pin> {
    pin: SharedPin<'a, ExtiInput<'a, P>>,
    timing: ManchesterTiming,
}

impl<'a, P: Pin> ManchesterReader<'a, P> {
    pub fn new(pin: SharedPin<'a, ExtiInput<'a, P>>, data_timing: Duration) -> Self {
        Self {
            pin,
            timing: create_manchester_timing(data_timing),
        }
    }

    #[inline]
    async fn read_byte(&mut self, decoder: &mut DecoderBool) -> Result<u8, ReadError> {
        let mut no_byte_iteration = 0u8;
        loop {
            Timer::after(self.timing.decoding_start_wait).await;
            if let Some(byte) = decoder.next(self.pin.is_high()) {
                debug!("We should not receive byte in this branch in manchester");
                Timer::after(self.timing.decoding_end_wait).await;
                return Ok(byte);
            }

            Timer::after(self.timing.decoding_middle_wait).await;
            let result = decoder.next(self.pin.is_high());
            Timer::after(self.timing.decoding_end_wait).await;

            if let Some(byte) = result {
                return Ok(byte);
            }

            no_byte_iteration += 1;
            if no_byte_iteration >= 8 {
                return Err(ReadError::OutOfTiming);
            }
        }
    }
}

impl<'a, P: Pin> BaseReader for ManchesterReader<'a, P> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        let mut decoder = DecoderBool::new(BitOrder::LittleEndian);
        for element in buffer.iter_mut() {
            *element = with_timeout(self.timing.decoding_timeout, self.read_byte(&mut decoder))
                .await
                .map_err(|_| ReadError::TimeoutError)??;
            // trace!("Manchester reader received byte = {:#04x}", element);
        }
        // FIXME this is wrong
        Ok(buffer.len())
    }
}
