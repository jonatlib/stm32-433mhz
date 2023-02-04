#![no_std]
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
