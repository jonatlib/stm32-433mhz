use crate::allocator::{AllocationHandler, Allocator, AllocatorError};
use box_ref::{SuperBoxRef, SuperBoxRefMut};
use core::marker::PhantomData;
use core::ops::Deref;

pub mod box_ref;

pub struct ColdBox<'a, T: ?Sized> {
    handler: AllocationHandler<'a>,
    _phantom: PhantomData<T>,
}

impl<'a, T> ColdBox<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    pub fn new<A>(value: T, allocator: &'a A) -> Result<Self, AllocatorError>
    where
        A: Allocator,
    {
        let handler = allocator.allocate(core::mem::size_of::<T>())?;
        let mut this = Self {
            handler,
            _phantom: Default::default(),
        };

        this.update(value)?;
        Ok(this)
    }

    pub fn to_owned(&self) -> Result<T, AllocatorError> {
        let mut buffer = [0u8; { core::mem::size_of::<T>() }];
        let read_size = self.handler.read_bytes(&mut buffer)?;

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

    ///
    /// This takes `value` by value ( :) ) on purpose
    /// so the caller have to pass the value here so it will be freed from stack
    /// If the caller want to keep the original value around he should copy it
    /// Or instead use `Self::as_ref`
    pub fn update(&mut self, mut value: T) -> Result<(), AllocatorError> {
        //  TODO see comments in `to_owned`
        let value_ptr = &mut value as *mut _ as *mut [u8; core::mem::size_of::<T>()];
        let value_bytes: [u8; core::mem::size_of::<T>()] = unsafe { value_ptr.read() };
        core::mem::forget(value);

        self.handler.write_bytes(&value_bytes)?;

        Ok(())
    }
}

impl<'a, T> ColdBox<'a, T>
where
    T: Sized,
    [(); core::mem::size_of::<T>()]:,
{
    pub fn try_borrow(&'a self) -> Result<SuperBoxRef<'a, T>, AllocatorError> {
        let value = self.to_owned()?;

        Ok(SuperBoxRef {
            value,
            handle: self,
        })
    }

    pub fn try_borrow_mut(&'a mut self) -> Result<SuperBoxRefMut<'a, T>, AllocatorError> {
        let value = self.to_owned()?;

        Ok(SuperBoxRefMut {
            value,
            handle: self,
        })
    }

    pub fn borrow(&'a self) -> SuperBoxRef<'a, T> {
        self.try_borrow().expect("Failed to read memory")
    }

    pub fn borrow_mut(&'a mut self) -> SuperBoxRefMut<'a, T> {
        self.try_borrow_mut().expect("Failed to read memory")
    }
}

impl<'a, T, const SIZE: usize> ColdBox<'a, [T; SIZE]>
where
    [(); core::mem::size_of::<T>()]:,
{
    pub fn get(&self, index: usize) -> Result<T, AllocatorError> {
        todo!()
    }

    pub fn set(&self, index: usize, value: T) -> Result<(), AllocatorError> {
        todo!()
    }

    // FIXME borrowing
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::allocator::{Allocator, DummyAllocator};
    use crate::memory::{DummyMemory, Memory};

    use std::boxed::Box;
    use std::vec::Vec;

    #[test]
    fn test_memory_footprint() {
        let memory = std::mem::size_of::<DummyMemory<[u8; 1]>>();
        let allocator = std::mem::size_of::<DummyAllocator<DummyMemory<[u8; 1]>>>();
        let our_box_1 = std::mem::size_of::<ColdBox<u8>>();
        let our_box_4 = std::mem::size_of::<ColdBox<u32>>();
        let our_box_128 = std::mem::size_of::<ColdBox<[u8; 128]>>();

        // println!("{}", memory);
        // println!("{}", allocator);
        // println!("{}", our_box_1);
        // println!("{}", our_box_4);
        // println!("{}", our_box_128);

        // These values will be platform dependant, we just test it is constant
        assert_eq!(memory, 1);
        assert_eq!(allocator, 32);
        assert_eq!(our_box_1, 32);
        assert_eq!(our_box_4, 32);
        assert_eq!(our_box_128, 32);
    }

    #[test]
    fn test_one_byte_type() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = Box::leak(Box::new(DummyAllocator::new(memory)));

        let boxed = ColdBox::new(5u8, allocator).unwrap();

        assert_eq!(boxed.to_owned().unwrap(), 5u8);
    }

    #[test]
    fn test_simple_type() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = DummyAllocator::new(memory);

        {
            let boxed = ColdBox::new(123456u32, &allocator).unwrap();
            assert_eq!(boxed.to_owned().unwrap(), 123456u32);
        }

        // println!("{:?}", allocator.collapse().collapse())
    }

    #[test]
    fn test_memory_size() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = DummyAllocator::new(memory);
        assert_eq!(allocator.available_memory(), 32);

        {
            let mut boxes = Vec::new();
            for index in 0u8..32 {
                let boxed = ColdBox::new(index, &allocator).unwrap();
                assert_eq!(boxed.to_owned().unwrap(), index);
                boxes.push(boxed);
            }
            assert_eq!(allocator.available_memory(), 0);

            let failing = ColdBox::new(0u8, &allocator);
            assert!(failing.is_err());
        }

        println!("{:?}", allocator.collapse().collapse())
    }

    #[test]
    fn test_multiple_allocations() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = DummyAllocator::new(memory);

        {
            let boxed1 = ColdBox::new(123456u32, &allocator).unwrap();
            assert_eq!(boxed1.to_owned().unwrap(), 123456u32);

            let boxed2 = ColdBox::new(456789u32, &allocator).unwrap();
            assert_eq!(boxed2.to_owned().unwrap(), 456789u32);
            // Test original memory is not corrupted
            assert_eq!(boxed1.to_owned().unwrap(), 123456u32);
        }

        println!("{:?}", allocator.collapse().collapse())
    }

    #[test]
    fn test_complex_type() {
        let memory = DummyMemory::new([0u8; 128]);
        let allocator = Box::leak(Box::new(DummyAllocator::new(memory)));

        struct Nested {
            value_1: i64,
            value_2: i128,
        };

        struct Testing {
            value_1: i64,
            value_2: i128,
            nested: Nested,
        };

        let value = Testing {
            value_1: -123456,
            value_2: 123456,
            nested: Nested {
                value_1: 123456,
                value_2: -123456,
            },
        };
        let boxed = ColdBox::new(value, allocator).unwrap();
        let borrowed = boxed.borrow();

        assert_eq!(borrowed.value_1, -123456);
        assert_eq!(borrowed.value_2, 123456);

        assert_eq!(borrowed.nested.value_1, 123456);
        assert_eq!(borrowed.nested.value_2, -123456);
    }
}
