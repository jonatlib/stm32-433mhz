use crate::utils::SharedPin;
use defmt::export::str;

use crate::error::WriterError;
use crate::BaseWriter;
use embassy_stm32::gpio::{Output, Pin};
use embassy_time::{Duration, Timer};
use manchester_code::{Datagram, DatagramBigEndianIterator, Encoder};

pub struct ManchesterWriter<'a, P: Pin> {
    pin: SharedPin<'a, Output<'a, P>>,
    // FIXME remove
    encoder: Encoder<DatagramBigEndianIterator>,
    timing: Duration,
}

impl<'a, P: Pin> ManchesterWriter<'a, P> {
    pub fn new(
        pin: SharedPin<'a, Output<'a, P>>,
        encoder: Encoder<DatagramBigEndianIterator>,
        timing: Duration,
    ) -> Self {
        Self {
            pin,
            encoder,
            timing,
        }
    }
}

impl<'a, P: Pin> BaseWriter for ManchesterWriter<'a, P> {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        let mut string_array = [0u8; 129];
        let mut string_index = 0usize;
        // let mut datagram = Datagram::default();
        buffer
            .iter()
            .map(|byte| {
                (0u8..8)
                    .into_iter()
                    .map(move |index| ((byte >> index) & 0x01) > 0)
            })
            .flatten()
            .for_each(|bit| {
                if string_index >= 128 {
                    panic!("Can't store more then 128bits in a single datagram");
                }

                //FIXME call private `add_bit` on datagram instead
                string_array[string_index] = (if bit { '1' } else { '0' }) as u8;
                string_index += 1;
            });

        let bits_string =
            core::str::from_utf8(&string_array).map_err(|_| WriterError::RuntimeError)?;

        let datagram = Datagram::new(bits_string);
        let encoder: Encoder<DatagramBigEndianIterator> = Encoder::new(datagram);
        for bit in encoder {
            if bit {
                self.pin.set_high();
            } else {
                self.pin.set_low();
            }
            Timer::after(self.timing).await;
        }

        Ok(buffer.len())
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        todo!()
    }
}
