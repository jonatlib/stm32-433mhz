use core::borrow::Borrow;
use core::marker::PhantomData;

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

#[derive(Debug)]
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

impl AllocationHandler {
    pub fn read_bytes(&self, buffer: &mut [u8]) -> Result<usize, AllocatorError> {
        self.handle.read_bytes(self, buffer)
    }

    pub fn write_bytes(&self, data: &[u8]) -> Result<usize, AllocatorError> {
        self.handle.write_bytes(self, data)
    }
}

impl Drop for AllocationHandler {
    fn drop(&mut self) {
        self.handle
            .free(self)
            .expect("Removing used memory should be without error");
    }
}

pub struct SuperBox<T: Sized> {
    handler: AllocationHandler,
    _phantom: PhantomData<T>,
}

impl<T: Sized> SuperBox<T> {}

impl<T: Sized> Borrow<T> for SuperBox<T>
where
    [(); core::mem::size_of::<T>()]: Sized,
{
    fn borrow<'a>(&'a self) -> &'a T {
        let mut buffer = [0u8; { core::mem::size_of::<T>() }];
        let read_size = self
            .handler
            .read_bytes(&mut buffer)
            .expect("Could not read data from memory");

        debug_assert_eq!(read_size, buffer.len());

        unsafe {
            let ptr = buffer.as_ptr();

            &*(ptr as *const T)
        }
    }
}
