use crate::transport::reader::TransportReader;

use crate::simple::codec::{
    create_codec, create_compression, CodecFactoryType, CompressionFactoryType,
};
use crate::Address;
use bit_io::BaseReader;
use codec::Codec;

pub struct SimpleReceiver<R, C, P> {
    address: Address,
    reader: R,
    codec: C,
    compression: P,
}

impl<R, C, P> SimpleReceiver<R, C, P>
where
    R: BaseReader,
    C: Codec,
    P: Codec,
{
    pub fn new(address: Address, reader: R, codec: C, compression: P) -> Self {
        Self {
            address,
            reader,
            codec,
            compression,
        }
    }

    pub fn create_transport(&mut self) -> TransportReader<R, C, P> {
        TransportReader::new(
            self.address.clone(),
            &self.codec,
            &self.compression,
            &mut self.reader,
        )
    }
}

impl<R> SimpleReceiver<R, CodecFactoryType, CompressionFactoryType>
where
    R: BaseReader,
{
    pub fn new_simple(address: Address, reader: R) -> Self {
        Self {
            address,
            reader,
            codec: create_codec(),
            compression: create_compression(),
        }
    }
}
