use core::borrow::Borrow;
use core::marker::PhantomData;
use core::ops::Deref;

pub type Address = usize;

/// Size in bytes
pub type Size = usize;

#[derive(Debug)]
pub enum MemoryError {
    ReadError,
    WriteError,
}

pub trait Memory<UNIT>
where
    UNIT: Sized + Borrow<u8>,
{
    type ReadIterator: Iterator<Item = UNIT>;
    type WriteIterator: Iterator<Item = UNIT>;

    fn read(&self, address: Address) -> Result<UNIT, MemoryError>;
    fn write(&mut self, address: Address, value: UNIT) -> Result<(), MemoryError>;

    fn read_page(&self, start: Address) -> Result<Self::ReadIterator, MemoryError>;
    fn write_page(&self, start: Address, value: Self::WriteIterator) -> Result<(), MemoryError>;

    fn read_slice(&self, address: &[Address]) -> Result<Self::ReadIterator, MemoryError>;
    fn write_slice(
        &self,
        address: &[Address],
        value: Self::WriteIterator,
    ) -> Result<(), MemoryError>;
}
