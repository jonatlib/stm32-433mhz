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
        match value {
            MemoryError::OutOfBound => Self::OOM,
            memory_error => Self::MemoryError(memory_error),
        }
    }
}

pub trait Allocator {
    fn allocate(&'static self, size: Size) -> Result<AllocationHandler, AllocatorError>;
    fn free(&self, handler: &AllocationHandler) -> Result<(), AllocatorError>;

    fn total_memory(&self) -> usize;
    fn available_memory(&self) -> usize;

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

pub struct DummyAllocator<M> {
    memory: RefCell<M>,
}

impl<M> DummyAllocator<M>
where
    M: Memory,
{
    pub fn new(memory: M) -> Self {
        Self {
            memory: RefCell::new(memory),
        }
    }

    pub fn collapse(self) -> M {
        self.memory
    }
}

impl<M> Allocator for DummyAllocator<M>
where
    M: Memory,
{
    fn allocate(&'static self, size: Size) -> Result<AllocationHandler, AllocatorError> {
        // FIXME this is wrong...
        Ok(AllocationHandler {
            size,
            start_address: 0,

            handle: self,
        })
    }

    fn free(&self, handler: &AllocationHandler) -> Result<(), AllocatorError> {
        // FIXME
        // todo!("free not implemented yet")
        Ok(())
    }

    fn total_memory(&self) -> usize {
        self.memory.borrow().available_memory()
    }

    fn available_memory(&self) -> usize {
        0
    }

    fn read_bytes(
        &self,
        handler: &AllocationHandler,
        buffer: &mut [u8],
    ) -> Result<usize, AllocatorError> {
        let addresses = handler.start_address..(handler.start_address + handler.size);
        Ok(self.memory.borrow().read_slice(addresses, buffer)?)
    }

    fn write_bytes(
        &self,
        handler: &AllocationHandler,
        data: &[u8],
    ) -> Result<usize, AllocatorError> {
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

    use std::boxed::Box;

    #[test]
    fn test_dummy_allocator() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = Box::leak(Box::new(DummyAllocator::new(memory)));
        // TODO maybe dont rely on 'static?
        // let allocator = DummyAllocator::new(memory);

        let handler = allocator.allocate(8).unwrap();
        let mut read_buffer = [0u8; 8];
        let expected_buffer = [0u8; 8];
        let write_buffer = [0u8, 1, 2, 3, 4, 5, 6, 7];

        handler.read_bytes(&mut read_buffer).unwrap();
        assert_eq!(read_buffer, expected_buffer);

        handler.write_bytes(&write_buffer).unwrap();
        handler.read_bytes(&mut read_buffer).unwrap();
        assert_eq!(read_buffer, write_buffer);
    }
}
