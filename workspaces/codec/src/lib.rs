#![no_std]
#![feature(type_alias_impl_trait)]

pub trait Codec {
    type Encoded<'a>: Iterator<Item = u8> + 'a;
    type Decoded<'a>: Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a>;
    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a>;

    fn get_encode_size(payload_size: usize) -> usize;
}

pub struct Identity {}

impl Default for Identity {
    fn default() -> Self {
        Self {}
    }
}

impl Codec for Identity {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        payload.into_iter().map(|v| *v)
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        payload.into_iter().map(|v| *v)
    }

    fn get_encode_size(payload_size: usize) -> usize {
        payload_size
    }
}
