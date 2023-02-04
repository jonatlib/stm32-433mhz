use crate::sync::SyncSequence;
use crate::{PinReader, Reader};

use crate::error::ReadError;
use crate::reader::ReaderTiming;
use embassy_stm32::gpio::Pin;

pub struct SyncReader<R: Reader> {
    reader: R,
    sync: SyncSequence,
    number_of_bytes: usize,
}

impl<R: Reader> SyncReader<R> {
    pub fn new(mut reader: R, sync: SyncSequence, number_of_bytes: usize) -> Self {
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

    pub fn get_timing(&self) -> &ReaderTiming {
        &self.reader.get_timing()
    }
}
