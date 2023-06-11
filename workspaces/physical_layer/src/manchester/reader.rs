use crate::error::ReadError;
use crate::utils::SharedPin;
use crate::BaseReader;

use crate::manchester::codec::{create_manchester_timing, ManchesterTiming};
use embassy_stm32::gpio::{Input, Pin};
use embassy_time::Duration;

pub struct ManchesterReader<'a, P: Pin> {
    pin: SharedPin<'a, Input<'a, P>>,
    timing: ManchesterTiming,
}

impl<'a, P: Pin> ManchesterReader<'a, P> {
    pub fn new(pin: SharedPin<'a, Input<'a, P>>, data_timing: Duration) -> Self {
        Self {
            pin,
            timing: create_manchester_timing(data_timing),
        }
    }
}

impl<'a, P: Pin> BaseReader for ManchesterReader<'a, P> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        todo!()
    }
}
