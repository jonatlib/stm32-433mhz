#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod error;
pub mod reader;
pub mod sync;
pub mod writer;

pub use reader::{PinReader, Reader};
pub use writer::{PinWriter, Writer};

pub use sync::sync_reader::SyncReader;
pub use sync::sync_writer::SyncWriter;
pub use sync::SyncSequence;

pub trait BaseReader {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, error::ReadError>;
}

pub trait BaseWriter {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, error::WriterError>;
}
