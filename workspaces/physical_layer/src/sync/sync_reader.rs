use crate::sync::SyncSequence;
use crate::Reader;

use crate::error::ReadError;
use crate::reader::ReaderTiming;

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

    pub fn get_timing(&self) -> &ReaderTiming {
        self.reader.get_timing()
    }
}

impl<R: Reader> crate::BaseReader for SyncReader<R> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        // TODO check if buffer is smaller then read bytes
        self.sync.read_sequence(&mut self.reader).await?;
        // FIXME check original line here...
        self.reader.read_bytes(buffer.len(), buffer).await
    }
}
