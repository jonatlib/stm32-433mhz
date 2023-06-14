use defmt::export::str;
use defmt::trace;
use embassy_stm32::gpio::{Output, Pin};
use embassy_time::{Duration, Timer};

use crate::error::WriterError;
use crate::utils::SharedPin;
use crate::BaseWriter;
use manchester::{create_manchester_timing, BitOrder, EncoderBoolIterator, ManchesterTiming};

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
        // trace!("Manchester writer writing buffer = {:?}", buffer);
        self.write_bytes_iterator(buffer.iter().copied()).await
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        let mut tmp_buffer = [0u8; 32];
        let mut elements = 0usize;
        data.enumerate().for_each(|(index, v)| {
            tmp_buffer[index] = v;
            elements = index + 1;
        });
        // trace!(
        //     "Manchester writer writing buffer = {:#04x}",
        //     &tmp_buffer[..elements]
        // );
        let data = tmp_buffer[..elements].into_iter().copied();

        let mut encoder = EncoderBoolIterator::new(data, BitOrder::LittleEndian);
        // FIXME use DMA instead and implement different encoder
        for bit in encoder {
            if bit {
                self.pin.set_high();
            } else {
                self.pin.set_low();
            }
            Timer::after(self.timing.encoding_between_half_bits).await;
        }
        self.pin.set_low();
        Ok(1) // FIXME
    }
}
