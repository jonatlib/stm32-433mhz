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

    pub fn get_timing(&self) -> &WriterTiming {
        self.writer.get_timing()
    }
}

impl<W: Writer> crate::BaseWriter for SyncWriter<W> {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.sync.write_sequence(&mut self.writer).await?;
        self.writer.write_bytes(buffer).await
    }
}
