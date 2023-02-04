use super::Codec;

impl<T> Codec for T
where
    T: Iterator<Item = u8>
{
    type Encoded = impl Iterator<Item = u8>;

    fn encode(self) -> Self::Encoded {
        self
            .map(|v| v * 2)
    }
}
