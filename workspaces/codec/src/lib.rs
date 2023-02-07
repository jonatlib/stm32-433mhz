#![no_std]
#![feature(type_alias_impl_trait, const_trait_impl)]

#[const_trait]
pub trait CodecSize {
    // TODO this method should return number bigger or equal to runtime size version of this function
    fn get_encode_const_size(payload_size: usize) -> usize;
}

pub trait Codec: CodecSize {
    type Encoded<'a>: Iterator<Item = u8> + 'a;
    type Decoded<'a>: Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a>;
    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a>;

    fn get_encode_size(payload_size: usize) -> usize;
}

#[derive(Default)]
pub struct Identity {}

impl Codec for Identity {
    type Encoded<'a> = impl Iterator<Item = u8> + 'a;
    type Decoded<'a> = impl Iterator<Item = u8> + 'a;

    fn encode<'a>(&self, payload: &'a [u8]) -> Self::Encoded<'a> {
        payload.iter().copied()
    }

    fn decode<'a>(&self, payload: &'a [u8]) -> Self::Decoded<'a> {
        payload.iter().copied()
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
