use embassy_time::{Duration, Timer};

use crate::error::WriterError;
use crate::BaseWriter;

use super::SyncMarkerWriter;

pub struct SyncWriter<W: BaseWriter, SW: SyncMarkerWriter> {
    sync: SW,
    writer: W,
    time_after_sync: Duration,
}

impl<W: BaseWriter, SW: SyncMarkerWriter> SyncWriter<W, SW> {
    pub fn new(sync: SW, writer: W, time_after_sync: Duration) -> Self {
        Self {
            sync,
            writer,
            time_after_sync,
        }
    }
}

impl<W: BaseWriter, SW: SyncMarkerWriter> BaseWriter for SyncWriter<W, SW> {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.sync.write_sync().await?;
        Timer::after(self.time_after_sync).await;
        self.writer.write_bytes_buffer(buffer).await
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        self.sync.write_sync().await?;
        Timer::after(self.time_after_sync).await;
        self.writer.write_bytes_iterator(data).await
    }
}
