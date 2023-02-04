use crate::sync::SyncSequence;
use crate::Writer;

use crate::error::WriterError;
use crate::writer::WriterTiming;

pub struct SyncWriter<W: Writer> {
    writer: W,
    sync: SyncSequence,
}

impl<W: Writer> SyncWriter<W> {
    pub fn new(writer: W, sync: SyncSequence) -> Self {
        Self { writer, sync }
    }

    pub async fn write_bytes(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.sync.write_sequence(&mut self.writer).await?;
        self.writer.write_bytes(buffer).await
    }

    pub fn get_timing(&self) -> &WriterTiming {
        self.writer.get_timing()
    }
}
