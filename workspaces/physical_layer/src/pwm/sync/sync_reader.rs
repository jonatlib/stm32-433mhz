use crate::pwm::reader::PwmReader;
use crate::pwm::reader::ReaderTiming;
use crate::pwm::sync::SyncSequence;

use crate::error::ReadError;

pub struct SyncReader<R: PwmReader> {
    reader: R,
    sync: SyncSequence,
    number_of_bytes: usize,
}

impl<R: PwmReader> SyncReader<R> {
    pub fn new(mut reader: R, sync: SyncSequence, number_of_bytes: usize) -> Self {
        reader.get_mut_timing().adjust_to_sync_marker(&sync);
        Self {
            sync,
            reader,
            number_of_bytes,
        }
    }

    pub fn get_timing(&self) -> &ReaderTiming {
        self.reader.get_timing()
    }
}

impl<R: PwmReader> crate::BaseReader for SyncReader<R> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        // TODO check if buffer is smaller then read bytes
        self.sync.read_sequence(&mut self.reader).await?;
        // FIXME check original line here...
        self.reader.read_bytes(buffer.len(), buffer).await
    }
}
