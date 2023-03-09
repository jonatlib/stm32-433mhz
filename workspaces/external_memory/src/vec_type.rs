use crate::allocator::{AllocationHandler, Allocator, AllocatorError};

use crate::vec_type::iterator::ColdVecIter;
use core::marker::PhantomData;
use core::ops::Range;

pub mod iterator;

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

    pub fn get_range(&self, range: Range<usize>) -> Result<&[T], AllocatorError> {
        // FIXME don't use `get` but read a range of memory instead
        // FIXME use some helper struct to keep range values
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

    // TODO use atomic instead?
    len: usize,

    allocator: &'a dyn Allocator,
}

impl<'a, T> ColdVec<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    const DEFAULT_SIZE: usize = 32;
    // Grow about 20% and shrink when 30% of capacity is not used
    // (but keep 20% buffer - so shrink about 10%)
    const GROW_FACTOR: usize = 20; // This means 20%
    const SHRINK_FACTOR: usize = 30; // This means 30%

    pub fn new(allocator: &'a dyn Allocator) -> Result<Self, AllocatorError> {
        Self::with_capacity(Self::DEFAULT_SIZE, allocator)
    }

    pub fn with_capacity(
        capacity: usize,
        allocator: &'a dyn Allocator,
    ) -> Result<Self, AllocatorError> {
        Ok(Self {
            data: RawColdVec::new(capacity, allocator)?,
            len: 0,

            allocator,
        })
    }

    pub fn push(&mut self, element: T) -> Result<(), AllocatorError> {
        // TODO do we want to do this check here? (other method)
        if self.len() + 1 > self.capacity() {
            self.grow_default()?;
        }

        self.data.set(self.len, element)?;
        self.len += 1;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Result<Option<T>, AllocatorError> {
        if index >= self.len() {
            return Ok(None);
        }

        Ok(Some(self.data.get(index)?))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.data.size
    }

    pub fn iter(&'a self) -> impl Iterator<Item = T> + 'a {
        ColdVecIter {
            index: 0,
            vector: &self,
        }
    }
}

impl<'a, T> ColdVec<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    fn grow_default(&mut self) -> Result<(), AllocatorError> {
        // FIXME
        let capacity = 100;
        self.grow(capacity)
    }

    fn shrink_default(&mut self) -> Result<(), AllocatorError> {
        // FIXME
        let capacity = 100;
        self.shrink(capacity)
    }

    fn grow(&mut self, new_capacity: usize) -> Result<(), AllocatorError> {
        let old_data = core::mem::replace(
            &mut self.data,
            RawColdVec::new(new_capacity, self.allocator)?,
        );

        // TODO use range operation instead
        for index in 0..self.len {
            self.data.set(index, old_data.get(index)?)?;
        }

        Ok(())
    }

    fn shrink(&mut self, new_capacity: usize) -> Result<(), AllocatorError> {
        todo!()
    }
}

impl<'a, T> FromIterator<T> for ColdVec<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = Self::new(crate::get_global_allocator()).expect("Could not allocate memory");
        for element in iter {
            data.push(element).expect("Could not add element to Vector");
        }
        data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::allocator::{Allocator, DummyAllocator};
    use crate::memory::{Address, DummyMemory, Memory, Size};

    use std::sync::{Arc, Mutex};

    /// Some structures used only in testing
    mod testing_structure {
        use crate::allocator::{AllocationHandler, Allocator, AllocatorError, DummyAllocator};
        use crate::memory::{Address, DummyMemory, Memory, Size};

        use std::sync::{Arc, Mutex};

        pub struct LockedAllocator<A: Allocator>(pub Arc<Mutex<A>>);

        impl<A: Allocator> Allocator for LockedAllocator<A> {
            fn allocate(&self, size: Size) -> Result<AllocationHandler, AllocatorError> {
                // self.0.lock().unwrap().allocate(size)
                todo!()
            }

            fn free(&self, handler: &AllocationHandler) -> Result<(), AllocatorError> {
                self.0.lock().unwrap().free(handler)
            }

            fn total_memory(&self) -> usize {
                self.0.lock().unwrap().total_memory()
            }

            fn available_memory(&self) -> usize {
                self.0.lock().unwrap().available_memory()
            }

            fn read_all_bytes(
                &self,
                handler: &AllocationHandler,
                buffer: &mut [u8],
            ) -> Result<usize, AllocatorError> {
                self.0.lock().unwrap().read_all_bytes(handler, buffer)
            }

            fn write_all_bytes(
                &self,
                handler: &AllocationHandler,
                data: &[u8],
            ) -> Result<usize, AllocatorError> {
                self.0.lock().unwrap().write_all_bytes(handler, data)
            }

            fn read_bytes(
                &self,
                handler: &AllocationHandler,
                offset_address: Address,
                buffer: &mut [u8],
            ) -> Result<usize, AllocatorError> {
                self.0
                    .lock()
                    .unwrap()
                    .read_bytes(handler, offset_address, buffer)
            }

            fn write_bytes(
                &self,
                handler: &AllocationHandler,
                offset_address: Address,
                data: &[u8],
            ) -> Result<usize, AllocatorError> {
                self.0
                    .lock()
                    .unwrap()
                    .write_bytes(handler, offset_address, data)
            }
        }
    }

    #[test]
    fn test_basic_operations() {
        let memory = DummyMemory::new([0u8; 4 * 4]);
        let allocator = DummyAllocator::new(memory);

        {
            let mut vector: ColdVec<u32> = ColdVec::with_capacity(4, &allocator).unwrap();
            vector.push(123456).unwrap();
            vector.push(789013).unwrap();
            vector.push(456789).unwrap();

            assert_eq!(vector.get(0).unwrap().unwrap(), 123456);
            assert_eq!(vector.get(1).unwrap().unwrap(), 789013);
            assert_eq!(vector.get(2).unwrap().unwrap(), 456789);
            assert_eq!(vector.get(3).unwrap(), None);

            let result1 = vector.push(123456);
            assert!(result1.is_ok());
            let result2 = vector.push(123456);
            assert!(result2.is_err());
        }

        println!("{:?}", allocator.collapse().collapse())
    }

    #[test]
    fn test_from_iter() {
        let memory = DummyMemory::new([0u8; 4 * 4]);
        let dummy_allocator = DummyAllocator::new(memory);
        let allocator = testing_structure::LockedAllocator(Arc::new(Mutex::new(dummy_allocator)));
        let static_ref_allocator: &'static testing_structure::LockedAllocator<
            DummyAllocator<DummyMemory<[u8; 16]>>,
        > = unsafe {
            &*((&std::mem::ManuallyDrop::new(allocator)) as *const _
                as *const testing_structure::LockedAllocator<_>)
        };

        crate::init_global_allocator(static_ref_allocator);

        {
            let mut vector: ColdVec<u32> = ColdVec::with_capacity(4, static_ref_allocator).unwrap();
            vector.push(123456).unwrap();
            vector.push(789013).unwrap();
            vector.push(456789).unwrap();
        }
    }
}
