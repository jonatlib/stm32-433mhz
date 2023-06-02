use embassy_time::Timer;

use crate::error::WriterError;
use crate::pwm::sync::SyncSequence;
use crate::pwm::writer::PwmWriter;
use crate::pwm::writer::WriterTiming;

pub struct SyncPwmWriter<W: PwmWriter> {
    writer: W,
    sync: SyncSequence,
}

impl<W: PwmWriter> SyncPwmWriter<W> {
    pub fn new(writer: W, sync: SyncSequence) -> Self {
        Self { writer, sync }
    }
}

impl<W: PwmWriter> crate::BaseWriter for SyncPwmWriter<W> {
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

            if let Some(between_bytes) = self.writer.get_timing().between_bytes {
                Timer::after(between_bytes).await;
            }
        }

        // Let some time between streams
        Timer::after(self.writer.get_timing().ones * 4).await;

        Ok(bytes)
    }
}
