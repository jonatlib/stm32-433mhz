use crate::error::{ReadError, WriterError};

pub mod reader;
pub mod writer;

pub trait SyncMarkerRead {
    async fn sync(&self) -> Result<(), ReadError>;
}

pub trait SyncMarkerWriter {
    async fn write_sync(&self) -> Result<(), WriterError>;
}
