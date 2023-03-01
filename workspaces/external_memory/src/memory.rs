use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;
use core::ops::Deref;
use core::ops::Range;

pub type Address = usize;

/// Size in bytes
pub type Size = usize;

#[derive(Debug)]
pub enum MemoryError {
    ReadError,
    WriteError,
    OutOfBound,
    Timeout,
}

pub trait Memory {
    fn read(&self, address: Address) -> Result<u8, MemoryError>;
    fn write(&mut self, address: Address, value: u8) -> Result<(), MemoryError>;

    fn read_page(&self, start: Address, buffer: &mut [u8]) -> Result<usize, MemoryError>;

    fn write_page<I, E>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = E>,
        E: Borrow<u8>;

    fn available_memory(&self) -> usize;
    fn page_size(&self) -> usize;
    fn available_pages(&self) -> usize;

    fn read_slice(&self, address: Range<usize>, buffer: &mut [u8]) -> Result<usize, MemoryError> {
        for (index, address) in address.enumerate() {
            buffer[index] = self.read(address)?;
        }

        // FIXME
        Ok(0)
    }

    fn write_slice<I, E>(&mut self, address: Range<usize>, value: I) -> Result<usize, MemoryError>
    where
        I: Iterator<Item = E>,
        E: Borrow<u8>,
    {
        for (address, data) in address.zip(value) {
            self.write(address, data.borrow().clone())?;
        }

        // FIXME
        Ok(0)
    }
}

pub struct DummyMemory<M> {
    memory: M,
}

impl<M> DummyMemory<M>
where
    M: Borrow<[u8]> + BorrowMut<[u8]>,
{
    pub fn new(memory: M) -> Self {
        Self { memory }
    }

    pub fn collapse(self) -> M {
        self.memory
    }
}

impl<M> Memory for DummyMemory<M>
where
    M: Borrow<[u8]> + BorrowMut<[u8]>,
{
    fn read(&self, address: Address) -> Result<u8, MemoryError> {
        self.memory
            .borrow()
            .get(address)
            .ok_or(MemoryError::OutOfBound)
            .copied()
    }

    fn write(&mut self, address: Address, value: u8) -> Result<(), MemoryError> {
        *(self
            .memory
            .borrow_mut()
            .get_mut(address)
            .ok_or(MemoryError::OutOfBound)?) = value.clone();

        Ok(())
    }

    fn read_page(&self, start: Address, buffer: &mut [u8]) -> Result<usize, MemoryError> {
        todo!()
    }

    fn write_page<I, E>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = E>,
        E: Borrow<u8>,
    {
        todo!()
    }

    fn available_memory(&self) -> usize {
        0
    }

    fn page_size(&self) -> usize {
        0
    }

    fn available_pages(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dummy_memory() {
        let mut memory = DummyMemory::new([0u8; 32]);

        let read_value = memory.read(0x00).unwrap();
        assert_eq!(read_value, 0);
        let read_value = memory.read(0x10).unwrap();
        assert_eq!(read_value, 0);

        memory.write(0x10, 25).unwrap();

        let read_value = memory.read(0x00).unwrap();
        assert_eq!(read_value, 0);
        let read_value = memory.read(0x10).unwrap();
        assert_eq!(read_value, 25);
    }
}
