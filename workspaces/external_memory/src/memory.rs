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
}

pub trait Memory<'a, UNIT>
where
    UNIT: Sized + Clone + Borrow<u8> + 'a,
    Self: 'a,
{
    fn read(&'a self, address: Address) -> Result<UNIT, MemoryError>;
    fn write(&mut self, address: Address, value: UNIT) -> Result<(), MemoryError>;

    fn read_page(&self, start: Address, buffer: &mut [UNIT]) -> Result<usize, MemoryError>;

    fn write_page<I>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = UNIT>;

    fn read_slice(
        &'a self,
        address: Range<usize>,
        buffer: &mut [UNIT],
    ) -> Result<usize, MemoryError> {
        for index in address {
            buffer[index] = self.read(index)?;
        }

        // FIXME
        Ok(0)
    }

    fn write_slice<'i, I, E>(
        &mut self,
        address: Range<usize>,
        value: I,
    ) -> Result<usize, MemoryError>
    where
        I: Iterator<Item = &'i E>,
        E: Borrow<UNIT> + 'i,
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
}

impl<'a, M> Memory<'a, &'a u8> for DummyMemory<M>
where
    Self: 'a,
    M: Borrow<[u8]> + BorrowMut<[u8]> + 'a,
{
    fn read(&self, address: Address) -> Result<&'_ u8, MemoryError> {
        self.memory
            .borrow()
            .get(address)
            .ok_or(MemoryError::OutOfBound)
    }

    fn write(&mut self, address: Address, value: &u8) -> Result<(), MemoryError> {
        *(self
            .memory
            .borrow_mut()
            .get_mut(address)
            .ok_or(MemoryError::OutOfBound)?) = value.clone();

        Ok(())
    }

    fn read_page(&self, start: Address, buffer: &mut [&'a u8]) -> Result<usize, MemoryError> {
        todo!()
    }

    fn write_page<I>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = &'a u8>,
    {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dummy_memory() {
        let mut memory = DummyMemory::new([0u8; 32]);

        let read_value = memory.read(0x00).unwrap();
        assert_eq!(*read_value, 0);
        let read_value = memory.read(0x10).unwrap();
        assert_eq!(*read_value, 0);

        memory.write(0x10, &25).unwrap();

        let read_value = memory.read(0x00).unwrap();
        assert_eq!(*read_value, 0);
        let read_value = memory.read(0x10).unwrap();
        assert_eq!(*read_value, 25);
    }
}
