use crate::sync::SyncSequence;
use crate::{PinWriter, Writer};

use crate::error::WriterError;
use crate::writer::WriterTiming;
use embassy_stm32::gpio::Pin;

pub struct SyncWriter<'a, P: Pin, const INVERT: bool = false> {
    writer: PinWriter<'a, P, INVERT>,
    sync: SyncSequence,
}

impl<'a, P: Pin, const INVERT: bool> SyncWriter<'a, P, INVERT> {
    pub fn new(writer: PinWriter<'a, P, INVERT>, sync: SyncSequence) -> Self {
        Self { writer, sync }
    }

    pub async fn write_bytes(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.sync.write_sequence(&mut self.writer).await?;
        self.writer.write_bytes(buffer).await
    }

    pub fn get_timing(&self) -> &WriterTiming {
        &self.writer.get_timing()
    }
}
