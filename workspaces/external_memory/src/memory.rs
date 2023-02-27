use core::borrow::{Borrow, BorrowMut};
use core::marker::PhantomData;
use core::ops::Deref;

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
    UNIT: Sized + Borrow<u8> + 'a,
    Self: 'a,
{
    type ReadIterator: Iterator<Item = UNIT> + 'a;

    fn read(&'a self, address: Address) -> Result<UNIT, MemoryError>;
    fn write(&mut self, address: Address, value: UNIT) -> Result<(), MemoryError>;

    fn read_page(&'a self, start: Address) -> Result<Self::ReadIterator, MemoryError>;

    fn write_page<I>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = UNIT>;

    fn read_slice(&self, address: &[Address]) -> Result<Self::ReadIterator, MemoryError> {
        todo!()
    }

    fn write_slice<I>(&mut self, address: &[Address], value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = UNIT>,
    {
        todo!()
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
    type ReadIterator = core::iter::Once<&'a u8>;

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

    fn read_page(&'a self, start: Address) -> Result<Self::ReadIterator, MemoryError> {
        Ok(core::iter::once(self.read(start)?))
    }

    fn write_page<I>(&mut self, start: Address, value: I) -> Result<(), MemoryError>
    where
        I: Iterator<Item = &'a u8>,
    {
        for (index, data) in value.enumerate() {
            self.write(start + index, data)?;
        }
        Ok(())
    }
}
