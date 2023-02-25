use core::marker::PhantomData;
use std::borrow::Borrow;

pub type Address = usize;

/// Size in bytes
pub type Size = usize;

pub enum MemoryError {
    ReadError,
    WriteError,
}

pub trait Memory<UNIT>
where
    UNIT: Sized + core::borrow::Borrow<u8>,
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

pub enum AllocatorError {
    MemoryError(MemoryError),
    OOM,
}

pub trait Allocator {
    fn allocate(&self, size: Size) -> Result<AllocationHandler, AllocatorError>;
    fn free(&self, handler: &AllocationHandler) -> Result<(), AllocatorError>;

    fn read_bytes(
        &self,
        handler: &AllocationHandler,
        buffer: &mut [u8],
    ) -> Result<usize, AllocatorError>;

    fn write_bytes(
        &self,
        handler: &AllocationHandler,
        data: &[u8],
    ) -> Result<usize, AllocatorError>;
}

pub struct AllocationHandler {
    size: Size,
    start_address: Address,

    handle: &'static dyn Allocator,
}

impl Drop for AllocationHandler {
    fn drop(&mut self) {
        self.handle.free(self)
    }
}

pub struct ExternalType<T: Sized> {
    handler: AllocationHandler,
    _phantom: PhantomData<T>,
}

impl<T: Sized> Borrow<T> for ExternalType<T> {
    fn borrow(&self) -> &T {
        todo!()
    }
}
