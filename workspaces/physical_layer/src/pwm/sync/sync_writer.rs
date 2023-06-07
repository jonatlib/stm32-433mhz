use embassy_time::Timer;

use crate::error::WriterError;
use crate::pwm::sync::SyncSequence;
use crate::pwm::writer::PwmWriter;
use crate::pwm::writer::WriterTiming;
use crate::sync::SyncMarkerWriter;

pub struct PwmSyncMarkerWriter<W: PwmWriter> {
    writer: W,
    sync: SyncSequence,
}

impl<W: PwmWriter> PwmSyncMarkerWriter<W> {
    pub fn new(writer: W, sync: SyncSequence) -> Self {
        Self { writer, sync }
    }
}

impl<W: PwmWriter> SyncMarkerWriter for PwmSyncMarkerWriter<W> {
    async fn write_sync(&mut self) -> Result<(), WriterError> {
        self.sync.write_sequence(&mut self.writer).await
    }
}
