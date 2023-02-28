use crate::allocator::{AllocationHandler, Allocator, AllocatorError};
use core::marker::PhantomData;
use core::ops::Deref;

pub struct SuperBox<T: Sized> {
    handler: AllocationHandler,
    _phantom: PhantomData<T>,
}

impl<T: Sized> SuperBox<T>
where
    [(); core::mem::size_of::<T>()]: Sized,
{
    pub fn new<A>(value: T, allocator: &'static mut A) -> Result<Self, AllocatorError>
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

    pub fn try_borrow(&self) -> Result<SuperBoxRef<'_, T>, AllocatorError> {
        let value = self.to_owned()?;

        Ok(SuperBoxRef {
            value,
            handle: self,
        })
    }

    pub fn try_borrow_mut(&mut self) -> Result<SuperBoxRefMut<'_, T>, AllocatorError> {
        let value = self.to_owned()?;

        Ok(SuperBoxRefMut {
            value,
            handle: self,
        })
    }

    pub fn borrow(&self) -> SuperBoxRef<'_, T> {
        self.try_borrow().expect("Failed to read memory")
    }

    pub fn borrow_mut(&mut self) -> SuperBoxRefMut<'_, T> {
        self.try_borrow_mut().expect("Failed to read memory")
    }
}

pub struct SuperBoxRef<'a, T: Sized> {
    value: T,
    handle: &'a SuperBox<T>,
}

pub struct SuperBoxRefMut<'a, T: Sized>
where
    [(); core::mem::size_of::<T>()]:,
{
    value: T,
    handle: &'a mut SuperBox<T>,
}

impl<'a, T: Sized> Deref for SuperBoxRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T: Sized> Drop for SuperBoxRefMut<'a, T>
where
    [(); core::mem::size_of::<T>()]:,
{
    fn drop(&mut self) {
        let value = unsafe { core::mem::replace(&mut self.value, core::mem::zeroed()) };
        self.handle
            .update(value)
            .expect("Memory could not be written");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::allocator::{Allocator, DummyAllocator};
    use crate::memory::{DummyMemory, Memory};

    use std::boxed::Box;

    #[test]
    fn test_one_byte_type() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = Box::leak(Box::new(DummyAllocator::new(memory)));

        let boxed = SuperBox::new(5u8, allocator).unwrap();

        assert_eq!(boxed.to_owned().unwrap(), 5u8);
    }

    #[test]
    fn test_simple_type() {
        let memory = DummyMemory::new([0u8; 32]);
        let allocator = Box::leak(Box::new(DummyAllocator::new(memory)));

        let boxed = SuperBox::new(123456u32, allocator).unwrap();

        assert_eq!(boxed.to_owned().unwrap(), 123456u32);
    }
}
