use defmt::export::str;
use embassy_stm32::gpio::{Output, Pin};
use embassy_time::{Duration, Timer};

use crate::error::WriterError;
use crate::manchester::codec::{
    create_manchester_timing, BitOrder, EncoderIterator, ManchesterTiming,
};
use crate::utils::SharedPin;
use crate::BaseWriter;

pub struct ManchesterWriter<'a, P: Pin> {
    pin: SharedPin<'a, Output<'a, P>>,
    timing: ManchesterTiming,
}

impl<'a, P: Pin> ManchesterWriter<'a, P> {
    pub fn new(pin: SharedPin<'a, Output<'a, P>>, data_timing: Duration) -> Self {
        Self {
            pin,
            timing: create_manchester_timing(data_timing),
        }
    }
}

impl<'a, P: Pin> BaseWriter for ManchesterWriter<'a, P> {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.write_bytes_iterator(buffer.iter().copied()).await
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        let mut encoder = EncoderIterator::new(data, BitOrder::LittleEndian);
        // FIXME use DMA instead and implement different encoder
        for bit in encoder {
            if bit {
                self.pin.set_high();
            } else {
                self.pin.set_low();
            }
            Timer::after(self.timing.encoding_between_half_bits).await;
        }
        Ok(1) // FIXME
    }
}
