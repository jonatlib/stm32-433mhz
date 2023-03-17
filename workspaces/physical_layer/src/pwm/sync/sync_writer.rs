use embassy_time::Timer;

use crate::error::WriterError;
use crate::pwm::sync::SyncSequence;
use crate::pwm::writer::WriterTiming;
use crate::pwm::Writer;

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

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        self.sync.write_sequence(&mut self.writer).await?;

        let mut bytes = 0usize;
        for byte in data {
            self.writer.write_byte(byte).await?;
            bytes += 1;

            if let Some(between_bytes) = self.get_timing().between_bytes {
                Timer::after(between_bytes).await;
            }
        }

        // Let some time between streams
        Timer::after(self.get_timing().ones * 4).await;

        Ok(bytes)
    }
}
