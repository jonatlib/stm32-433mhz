use crate::error::NetworkError;
use crate::packet::Packet32;

pub struct Window<const SIZE: usize> {
    buffer: heapless::Vec<Packet32, SIZE>,
}

impl<const SIZE: usize> Window<SIZE> {
    pub fn new() -> Self {
        Self {
            buffer: heapless::Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn push_packet(&mut self, packet: Packet32) -> Result<Option<usize>, NetworkError> {
        todo!()
    }

    pub fn write_buffer(&self, buffer: &mut [u8]) {
        todo!()
    }
}
