use crate::memory::{Address, Memory, MemoryError, Size};
use core::borrow::Borrow;
use core::cell::RefCell;
use core::marker::PhantomData;

#[derive(Debug)]
pub enum AllocatorError {
    MemoryError(MemoryError),
    OOM,
}

impl From<MemoryError> for AllocatorError {
    fn from(value: MemoryError) -> Self {
        todo!()
    }
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

pub struct DummyAllocator<'a, M, E> {
    memory: RefCell<M>,

    _phantom: PhantomData<&'a E>,
}

impl<'a, M, E> DummyAllocator<'a, M, E>
where
    M: Memory<'a, E>,
    E: Borrow<u8> + 'a,
{
    pub fn new(memory: M) -> Self {
        Self {
            memory: RefCell::new(memory),
            _phantom: Default::default(),
        }
    }
}

impl<'a, M, E> Allocator for DummyAllocator<'a, M, E>
where
    M: Memory<'a, E>,
    E: Borrow<u8> + 'a,
{
    fn allocate(&self, size: Size) -> Result<AllocationHandler, AllocatorError> {
        todo!()
    }

    fn free(&self, handler: &AllocationHandler) -> Result<(), AllocatorError> {
        todo!("free not implemented yet")
    }

    fn read_bytes(
        &self,
        handler: &AllocationHandler,
        buffer: &mut [u8],
    ) -> Result<usize, AllocatorError> {
        let index = 0;
        let addresses = handler.start_address..(handler.start_address + handler.size);
        for (index, data) in self.memory.borrow().read_slice(addresses)?.enumerate() {
            buffer[index] = *data.borrow();
        }
        Ok(index)
    }

    fn write_bytes(
        &self,
        handler: &AllocationHandler,
        data: &[u8],
    ) -> Result<usize, AllocatorError> {
        let index = 0;
        let addresses = handler.start_address..(handler.start_address + handler.size);
        Ok(self
            .memory
            .borrow_mut()
            .write_slice(addresses, data.into_iter())?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::memory::{DummyMemory, Memory};

    #[test]
    fn test_dummy_allocator() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = DummyAllocator::new(memory);
    }
}
