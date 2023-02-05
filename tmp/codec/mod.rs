pub mod identity;
#[cfg(feature = "ldpc")]
pub mod ldpc;
#[cfg(feature = "reedsolomon")]
pub mod reed_solomon;
#[cfg(feature = "sixbits")]
pub mod sixbits;

pub trait BytesCodec<T>: Default
where
    T: IntoIterator<Item = u8>,
{
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> T;
}

pub type ChainedData = heapless::Vec<u8, heapless::consts::U64>;
pub struct ChainedCodec<T, U, V>
where
    T: BytesCodec<ChainedData>,
    U: BytesCodec<ChainedData>,
    V: BytesCodec<ChainedData>,
{
    codec1: T,
    codec2: U,
    codec3: V,
}

pub type TwoChainedCodec<T, U> = ChainedCodec<T, U, identity::Identity>;

pub type Identity = TwoChainedCodec<identity::Identity, identity::Identity>;
#[cfg(feature = "ldpc")]
pub type Ldpc = TwoChainedCodec<ldpc::Ldpc, identity::Identity>;
#[cfg(feature = "reedsolomon")]
pub type ReedSolomon = TwoChainedCodec<reed_solomon::ReedSolomon, identity::Identity>;
#[cfg(feature = "sixbits")]
pub type SixBits = TwoChainedCodec<sixbits::SixBitsCodec, identity::Identity>;
#[cfg(all(feature = "ldpc", feature = "sixbits"))]
pub type LdpcSixBits = TwoChainedCodec<ldpc::Ldpc, sixbits::SixBitsCodec>;
#[cfg(all(feature = "reedsolomon", feature = "sixbits"))]
pub type ReedSolomonSixBits = TwoChainedCodec<reed_solomon::ReedSolomon, sixbits::SixBitsCodec>;

impl<T, U> TwoChainedCodec<T, U>
where
    T: BytesCodec<ChainedData>,
    U: BytesCodec<ChainedData>,
{
    pub fn new_2(c1: T, c2: U) -> Self {
        return Self {
            codec1: c1,
            codec2: c2,
            codec3: identity::Identity::default(),
        };
    }
}

impl<T, U, V> ChainedCodec<T, U, V>
where
    T: BytesCodec<ChainedData>,
    U: BytesCodec<ChainedData>,
    V: BytesCodec<ChainedData>,
{
    pub fn new_3(c1: T, c2: U, c3: V) -> Self {
        return Self {
            codec1: c1,
            codec2: c2,
            codec3: c3,
        };
    }
}

impl<T, U, V> Default for ChainedCodec<T, U, V>
where
    T: BytesCodec<ChainedData>,
    U: BytesCodec<ChainedData>,
    V: BytesCodec<ChainedData>,
{
    fn default() -> Self {
        return Self {
            codec1: T::default(),
            codec2: U::default(),
            codec3: V::default(),
        };
    }
}

impl<T, U, V> BytesCodec<ChainedData> for ChainedCodec<T, U, V>
where
    T: BytesCodec<ChainedData>,
    U: BytesCodec<ChainedData>,
    V: BytesCodec<ChainedData>,
{
    fn encode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let r1 = self.codec1.encode(input);
        let r2 = self.codec2.encode(r1);
        return self.codec3.encode(r2);
    }

    fn decode(&self, input: impl IntoIterator<Item = u8>) -> ChainedData {
        let r1 = self.codec3.decode(input);
        let r2 = self.codec2.decode(r1);
        return self.codec1.decode(r2);
    }
}
