use std::vec::Vec;

use physical_layer::error::{ReadError, WriterError};
use physical_layer::{BaseReader, BaseWriter};

pub fn prepare_io() -> (DummyManchesterReader, DummyManchesterWriter) {
    todo!()
}

pub struct DummyManchesterReader;

impl DummyManchesterReader {
    pub fn new() -> Self {
        todo!()
    }
}

impl BaseReader for DummyManchesterReader {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        todo!()
    }
}

pub struct DummyManchesterWriter;

impl DummyManchesterWriter {
    pub fn new() -> Self {
        todo!()
    }
}

impl BaseWriter for DummyManchesterWriter {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        todo!()
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        todo!()
    }
}
