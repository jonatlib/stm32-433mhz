#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, type_alias_impl_trait, const_trait_impl)]

mod packet;
mod packet_builder;
mod sequence_number;

pub mod error;
pub mod simple;
pub mod transport;

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
