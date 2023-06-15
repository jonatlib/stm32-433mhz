#![no_std]
#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    type_alias_impl_trait,
    const_trait_impl,
    generic_const_exprs
)]

// Enable testing on local machine
#[cfg(test)]
#[macro_use]
extern crate std;

mod packet;
mod packet_builder;

pub mod error;
pub mod simple;
pub mod transport;

#[cfg(not(any(feature = "packet-32", feature = "packet-64")))]
compile_error!("You need to enable one of the features `packet-32` or `packet-64`");

#[cfg(all(feature = "packet-32", feature = "packet-64"))]
compile_error!("You need to enable exactly one of the features `packet-32` or `packet-64`");

#[derive(Clone)]
pub struct Address {
    pub local_address: u8,
    pub destination_address: u8,
}

impl Address {
    pub fn new(local_address: u8, destination_address: u8) -> Self {
        Self {
            local_address,
            destination_address,
        }
    }
}
