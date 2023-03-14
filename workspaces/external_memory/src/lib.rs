#![no_std]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

// Enable testing on local machine
#[cfg(test)]
#[macro_use]
extern crate std;

pub type GlobalAllocator = &'static (dyn allocator::Allocator + Sync + 'static);

/// Global variable holding the global allocator used in cases where you can't pass it
static _GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR: AllocatorHandler =
    AllocatorHandler(core::cell::UnsafeCell::new(core::mem::MaybeUninit::uninit()));

/// Handler to have global allocator stored somewhere
struct AllocatorHandler(core::cell::UnsafeCell<core::mem::MaybeUninit<GlobalAllocator>>);
unsafe impl Sync for AllocatorHandler {}

/// Initialize global allocator
pub unsafe fn init_global_allocator(value: GlobalAllocator) {
    let global_ref = unsafe { &mut *_GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR.0.get() };
    *global_ref = core::mem::MaybeUninit::new(value);
}

pub unsafe fn uninit_global_allocator() {
    let global_ref = unsafe { &mut *_GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR.0.get() };
    *global_ref = core::mem::MaybeUninit::uninit();
}

/// Get current global allocator. `init_global_allocator` must be called before
pub unsafe fn get_global_allocator() -> GlobalAllocator {
    let uninit_alloc = unsafe { &*_GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR.0.get() };

    unsafe { *uninit_alloc.as_ptr() }
}

#[macro_export]
macro_rules! leak_and_init_global_allocator {
    ($allocator: expr, $typ: ty) => {
        let non_droppable: core::mem::ManuallyDrop<$typ> = core::mem::ManuallyDrop::new($allocator);
        let static_ref_allocator: &'static $typ =
            unsafe { &*((&non_droppable as *const _) as *const $typ) };
        core::mem::forget(non_droppable);

        unsafe { crate::init_global_allocator(static_ref_allocator) }
    };
}

pub mod allocator;
pub mod box_type;
pub mod memory;
pub mod vec_type;

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::ManuallyDrop;

    use crate::allocator::{Allocator, DummyAllocator};
    use crate::memory::{Address, DummyMemory, Memory, Size};

    use crate::vec_type::ColdVec;

    #[test]
    fn test_global_allocator_init() {
        let memory = DummyMemory::new([0u8; 16]);
        let allocator = DummyAllocator::new(memory);
        crate::leak_and_init_global_allocator!(allocator, DummyAllocator<DummyMemory<[u8; 16]>>);
        let static_ref_allocator = unsafe { crate::get_global_allocator() };

        {
            let mut vector: ColdVec<u32> = ColdVec::with_capacity(4, static_ref_allocator).unwrap();
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

        unsafe {
            crate::uninit_global_allocator();
        }
    }

    #[test]
    fn test_global_allocator_basic_test() {
        let memory = DummyMemory::new([0u8; 16]);
        let allocator = DummyAllocator::new(memory);

        // TODO make some function?
        let non_droppable = std::mem::ManuallyDrop::new(allocator);
        let original_address = std::ptr::addr_of!(non_droppable) as usize;
        let static_ref_allocator: &'static DummyAllocator<DummyMemory<[u8; 16]>> =
            unsafe { &*((&non_droppable as *const _) as *const DummyAllocator<_>) };
        std::mem::forget(non_droppable);

        unsafe {
            crate::init_global_allocator(static_ref_allocator);
        }
        let initialized_address = std::ptr::addr_of!(*static_ref_allocator) as usize;
        assert_eq!(original_address, initialized_address);

        {
            let alloc = unsafe { crate::get_global_allocator() };
            let alloc_address = alloc as *const _ as *const () as usize;
            assert_eq!(original_address, alloc_address);

            let mut vector: ColdVec<u32> = ColdVec::with_capacity(4, alloc).unwrap();
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

        unsafe {
            crate::uninit_global_allocator();
        }
    }
}
