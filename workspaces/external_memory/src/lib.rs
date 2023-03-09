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
pub fn init_global_allocator(value: GlobalAllocator) {
    let global_ref = unsafe { &mut *_GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR.0.get() };
    *global_ref = core::mem::MaybeUninit::new(value);
}

/// Get current global allocator. `init_global_allocator` must be called before
pub fn get_global_allocator() -> GlobalAllocator {
    unsafe { *(*_GLOBAL_DEFAULT_EXTERNAL_ALLOCATOR.0.get()).as_ptr() }
}

pub mod allocator;
pub mod box_type;
pub mod memory;
pub mod vec_type;
