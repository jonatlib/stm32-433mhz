use crate::transport::writer::TransportWriter;

use crate::Address;
use bit_io::BaseWriter;
use codec::Codec;

pub struct SimpleSender<W, C> {
    address: Address,
    writer: W,
    codec: C,
}

impl<W, C> SimpleSender<W, C>
where
    W: BaseWriter,
    C: Codec,
{
    pub fn new(address: Address, writer: W, codec: C) -> Self {
        Self {
            address,
            writer,
            codec,
        }
    }

    pub fn create_transport(&mut self) -> TransportWriter<W, C> {
        TransportWriter::new(self.address.clone(), 2, &self.codec, &mut self.writer)
    }
}

impl<W> SimpleSender<W, codec::Identity>
where
    W: BaseWriter,
{
    pub fn new_simple(address: Address, writer: W) -> Self {
        Self {
            address,
            writer,
            codec: codec::Identity::default(),
        }
    }
}
