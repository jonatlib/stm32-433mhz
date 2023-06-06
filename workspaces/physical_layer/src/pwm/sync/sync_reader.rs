use crate::pwm::reader::PwmReader;
use crate::pwm::reader::ReaderTiming;
use crate::pwm::sync::SyncSequence;
use core::cell::RefCell;
use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::error::ReadError;
use crate::sync::SyncMarkerRead;

pub struct PwmSyncMarkerReader<R: PwmReader> {
    reader: R,
    sync: SyncSequence,
    _phantom: PhantomData<R>,
}

impl<R: PwmReader> PwmSyncMarkerReader<R> {
    pub fn new(mut reader: R, sync: SyncSequence) -> Self {
        reader.get_mut_timing().adjust_to_sync_marker(&sync);
        Self {
            sync,
            reader,
            _phantom: Default::default(),
        }
    }
}

impl<R: PwmReader> SyncMarkerRead for PwmSyncMarkerReader<R> {
    async fn sync(&mut self) -> Result<(), ReadError> {
        self.sync.read_sequence(&mut self.reader).await
    }
}
