use crate::pwm::reader::PwmReader;
use crate::pwm::reader::ReaderTiming;
use crate::pwm::sync::SyncSequence;
use core::cell::RefCell;
use core::marker::PhantomData;
use core::ops::DerefMut;

use crate::error::ReadError;
use crate::sync::SyncMarkerRead;

pub struct PwmSyncMarkerReader<'a, R: PwmReader> {
    reader: &'a RefCell<R>,
    sync: SyncSequence,
    _phantom: PhantomData<R>,
}

impl<'a, R: PwmReader> PwmSyncMarkerReader<'a, R> {
    pub fn new(mut reader: &'a RefCell<R>, sync: SyncSequence) -> Self {
        reader
            .borrow_mut()
            .get_mut_timing()
            .adjust_to_sync_marker(&sync);
        Self {
            sync,
            reader,
            _phantom: Default::default(),
        }
    }
}

impl<R: PwmReader> SyncMarkerRead for PwmSyncMarkerReader<'_, R> {
    async fn sync(&mut self) -> Result<(), ReadError> {
        self.sync
            .read_sequence(self.reader.borrow_mut().deref_mut())
            .await
    }
}
