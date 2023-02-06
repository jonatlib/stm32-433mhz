use crate::transport::reader::TransportReader;

use crate::Address;
use bit_io::BaseReader;
use codec::Codec;

pub struct SimpleReceiver<R, C> {
    address: Address,
    reader: R,
    codec: C,
}

impl<R, C> SimpleReceiver<R, C>
where
    R: BaseReader,
    C: Codec,
{
    pub fn new(address: Address, reader: R, codec: C) -> Self {
        Self {
            address,
            reader,
            codec,
        }
    }

    pub fn create_transport(&mut self) -> TransportReader<R, C> {
        TransportReader::new(self.address.clone(), &self.codec, &mut self.reader)
    }
}

impl<R> SimpleReceiver<R, codec::Identity>
where
    R: BaseReader,
{
    pub fn new_simple(address: Address, reader: R) -> Self {
        Self {
            address,
            reader,
            codec: codec::Identity::default(),
        }
    }
}
