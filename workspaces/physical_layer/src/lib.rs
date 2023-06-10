#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

pub mod error;
pub mod manchester;
pub mod pwm;
pub mod sync;
pub mod utils;

pub trait BaseReader {
    async fn init(&mut self) {} // FIXME call inits before using reader (not inside the reader)
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, error::ReadError>;
}

pub trait BaseWriter {
    async fn init(&mut self) {} // FIXME call inits before using writer
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, error::WriterError>;
    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, error::WriterError>;

    // TODO these methods could be used on devices
    // TODO which have reader and writer at the same time
    // TODO to do a really basic collision avoidance
    fn pause_transmission(&mut self) {}
    fn resume_transmission(&mut self) {}
}
