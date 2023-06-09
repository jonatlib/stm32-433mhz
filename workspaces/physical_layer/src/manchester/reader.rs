use crate::error::ReadError;
use crate::utils::SharedPin;
use crate::BaseReader;

use embassy_stm32::gpio::{Input, Pin};
use embassy_time::Duration;
use manchester_code::Decoder;

pub struct ManchesterReader<'a, P: Pin> {
    pin: SharedPin<'a, Input<'a, P>>,
    // FIXME remove
    decoder: Decoder,
    timing: Duration,
}

impl<'a, P: Pin> ManchesterReader<'a, P> {
    pub fn new(pin: SharedPin<'a, Input<'a, P>>, decoder: Decoder, timing: Duration) -> Self {
        Self {
            pin,
            decoder,
            timing,
        }
    }
}

impl<'a, P: Pin> BaseReader for ManchesterReader<'a, P> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        loop {
            match self.decoder.next(self.pin.is_high()) {
                Some(v) => {
                    todo!()
                }
                None => (),
            }
        }
        todo!()
    }
}
