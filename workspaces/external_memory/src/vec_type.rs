use crate::allocator::{AllocationHandler, Allocator, AllocatorError};

use core::marker::PhantomData;
use core::ops::Range;

struct RawColdVec<'a, T: ?Sized> {
    handler: AllocationHandler<'a>,
    size: usize,

    _phantom: PhantomData<T>,
}

impl<'a, T> RawColdVec<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    pub fn new(size: usize, allocator: &'a dyn Allocator) -> Result<Self, AllocatorError> {
        Ok(Self {
            handler: allocator.allocate(size * core::mem::size_of::<T>())?,
            size,

            _phantom: Default::default(),
        })
    }

    pub fn get_range(&self, range: Range<usize>) -> Result<[T], AllocatorError> {
        todo!()
    }

    pub fn get(&self, index: usize) -> Result<T, AllocatorError> {
        let mut buffer = [0u8; { core::mem::size_of::<T>() }];
        let read_size = self
            .handler
            .read_bytes(index * core::mem::size_of::<T>(), &mut buffer)?;

        // FIXME dont disable this
        // debug_assert_eq!(read_size, buffer.len());

        // TODO this would be nice but not doable now because of
        //  https://github.com/rust-lang/rust/issues/61956
        // core::mem::transmute(buffer)

        // From https://github.com/rust-lang/rust/issues/61956
        let ptr = &mut buffer as *mut _ as *mut T;
        let result = unsafe { ptr.read() };
        core::mem::forget(buffer);
        Ok(result)
    }

    pub fn set(&mut self, index: usize, mut value: T) -> Result<(), AllocatorError> {
        //  TODO see comments in `to_owned`
        let value_ptr = &mut value as *mut _ as *mut [u8; core::mem::size_of::<T>()];
        let value_bytes: [u8; core::mem::size_of::<T>()] = unsafe { value_ptr.read() };
        core::mem::forget(value);

        self.handler
            .write_bytes(index * core::mem::size_of::<T>(), &value_bytes)?;

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct ColdVec<'a, T: ?Sized> {
    data: RawColdVec<'a, T>,
    len: usize,
}

impl<'a, T> ColdVec<'a, T>
where
    T: Sized,
{
    pub fn new(allocator: &'a dyn Allocator) -> Result<Self, AllocatorError> {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Result<Self, AllocatorError> {
        todo!()
    }

    pub fn push(&mut self, element: T) -> Result<(), AllocatorError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::allocator::{Allocator, DummyAllocator};
    use crate::memory::{DummyMemory, Memory};

    #[test]
    fn test_basic_operations() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = DummyAllocator::new(memory);

        let mut vector: ColdVec<u32> = ColdVec::new(&allocator).unwrap();
        vector.push(123456).unwrap();
        vector.push(789013).unwrap();
        vector.push(456789).unwrap();
    }
}
