#![no_std]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

// Enable testing on local machine
#[cfg(test)]
#[macro_use]
extern crate std;

pub mod allocator;
pub mod box_type;
pub mod memory;
