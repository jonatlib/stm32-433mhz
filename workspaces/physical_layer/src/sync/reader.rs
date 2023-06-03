use embassy_time::{Duration, Timer};

use crate::error::ReadError;
use crate::BaseReader;

use super::SyncMarkerRead;

pub struct SyncReader<R: BaseReader, SR: SyncMarkerRead> {
    sync: SR,
    reader: R,
    time_after_sync: Duration,
}

impl<R: BaseReader, SR: SyncMarkerRead> BaseReader for SyncReader<R, SR> {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        self.sync.sync().await?;
        Timer::after(self.time_after_sync).await;
        self.reader.read_bytes_buffer(buffer).await
    }
}
