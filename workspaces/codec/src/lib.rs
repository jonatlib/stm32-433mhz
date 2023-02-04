#![no_std]
#![feature(type_alias_impl_trait)]

pub mod two_to_three;

pub trait Codec
where Self: Iterator<Item = u8>
{
    type Encoded: Iterator<Item = u8>;

    fn encode(self) -> Self::Encoded;
}

pub fn example() -> u32 {
    123
}
