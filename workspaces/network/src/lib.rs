#![no_std]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait, type_alias_impl_trait)]

mod packet;
mod packet_builder;

pub mod error;
pub mod simple;
pub mod transport;

#[derive(Clone)]
pub struct Address {
    pub source_address: u8,
    pub destination_address: u8,
}

impl Address {
    pub fn new(source_address: u8, destination_address: u8) -> Self {
        Self {
            source_address,
            destination_address,
        }
    }
}
