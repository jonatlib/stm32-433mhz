use crate::error::ReadError;
use crate::BaseReader;

use manchester_code;

pub struct ManchesterReader {}

impl BaseReader for ManchesterReader {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        todo!()
    }
}
