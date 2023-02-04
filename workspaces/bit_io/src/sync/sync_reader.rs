use crate::sync::SyncSequence;
use crate::{PinReader, Reader};

use crate::error::ReadError;
use embassy_stm32::gpio::Pin;

pub struct SyncReader<'a, P: Pin, const INVERT: bool = false> {
    reader: PinReader<'a, P, INVERT>,
    sync: SyncSequence,
    number_of_bytes: usize,
}

impl<'a, P: Pin, const INVERT: bool> SyncReader<'a, P, INVERT> {
    pub fn new(
        mut reader: PinReader<'a, P, INVERT>,
        sync: SyncSequence,
        number_of_bytes: usize,
    ) -> Self {
        reader.get_mut_timing().adjust_to_sync_marker(&sync);
        Self {
            sync,
            reader,
            number_of_bytes,
        }
    }

    pub async fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        self.sync.read_sequence(&mut self.reader).await?;
        self.reader.read_bytes(self.number_of_bytes, buffer).await
    }
}
