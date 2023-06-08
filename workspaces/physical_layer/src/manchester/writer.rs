use crate::utils::SharedPin;
use defmt::export::str;

use crate::error::WriterError;
use crate::BaseWriter;
use embassy_stm32::gpio::{Output, Pin};
use embassy_time::Duration;
use manchester_code::{Datagram, DatagramBigEndianIterator, Encoder};

pub struct ManchesterWriter<'a, P: Pin> {
    pin: SharedPin<'a, Output<'a, P>>,
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
        let mut string = [0u8; 129];
        let mut index = 0usize;
        // let mut datagram = Datagram::default();
        buffer
            .iter()
            .map(|byte| {
                (0u8..8)
                    .into_iter()
                    .map(|index| ((byte >> index) & 0x01) > 0)
            })
            .flatten()
            .for_each(|bit| {
                if index >= 128 {
                    panic!("Can't store more then 128bits in a single datagram");
                }

                //FIXME call private `add_bit` on datagram instead
                string[index] = (if bit { '1' } else { '0' }).encode_utf8();
                index += 1;
            });

        todo!()
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        todo!()
    }
}
