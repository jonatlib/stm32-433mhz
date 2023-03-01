use crate::allocator::{AllocationHandler, Allocator, AllocatorError};

use core::marker::PhantomData;
use std::slice::SliceIndex;

struct RawColdVec<'a, T: ?Sized> {
    handle: AllocationHandler<'a>,
    size: usize,

    _phantom: PhantomData<T>,
}

impl<'a, T> RawColdVec<'a, T>
where
    T: Sized,
{
    pub fn new(size: usize, allocator: &'a Allocator) -> Result<Self, AllocatorError> {
        Ok(Self {
            handle: allocator.allocate(size * core::mem::size_of::<T>())?,
            size,

            _phantom: Default::default(),
        })
    }

    // TODO do we want to support range operations here?
    pub fn get(&self, index: usize) -> Result<T, AllocatorError> {
        todo!()
    }

    pub fn set(&mut self, index: usize, value: T) -> Result<(), AllocatorError> {
        todo!()
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
    pub fn new() -> Result<Self, AllocatorError> {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Result<Self, AllocatorError> {
        todo!()
    }
}
