#![no_std]
#![feature(
    type_alias_impl_trait,
    impl_trait_in_assoc_type,
    const_trait_impl,
    int_roundings
)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

// Enable testing on local machine
#[cfg(test)]
#[macro_use]
extern crate std;

use defmt::Format;

pub mod chain; // TODO, don't know what to do with this
pub mod four_to_six;
pub mod lzss;
pub mod reed_solomon;

#[derive(Format, Debug)]
pub enum CodecError {
    EncodeError,
    DecodeError,
}

#[const_trait]
pub trait CodecSize {
    // TODO this method should return number bigger or equal to runtime size version of this function
    fn get_encode_const_size(payload_size: usize) -> usize;
}

pub trait Codec: CodecSize {
    type Encoded<'a>: Iterator<Item = u8> + 'a;
    type Decoded<'a>: Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError>;
    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError>;

    fn get_encode_size(payload_size: usize) -> usize;
}

#[derive(Default)]
pub struct Identity {}

impl Codec for Identity {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Result<Self::Encoded<'a>, CodecError> {
        Ok(payload.iter().copied())
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Result<Self::Decoded<'a>, CodecError> {
        Ok(payload.iter().copied())
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size
    }
}

impl const CodecSize for Identity {
    fn get_encode_const_size(payload_size: usize) -> usize {
        payload_size
    }
}
