use crate::transport::writer::TransportWriter;

use crate::simple::codec::{
    create_codec, create_compression, CodecFactoryType, CompressionFactoryType,
};
use crate::Address;
use codec::Codec;
use physical_layer::BaseWriter;

pub struct SimpleSender<W, C, P> {
    address: Address,
    writer: W,
    codec: C,
    compression: P,
}

impl<W, C, P> SimpleSender<W, C, P>
where
    W: BaseWriter,
    C: Codec,
    P: Codec,
{
    pub fn new(address: Address, writer: W, codec: C, compression: P) -> Self {
        Self {
            address,
            writer,
            codec,
            compression,
        }
    }

    pub fn create_transport(&mut self) -> TransportWriter<W, C, P> {
        TransportWriter::new(
            self.address.clone(),
            3,
            &self.codec,
            &self.compression,
            &mut self.writer,
        )
    }
}

impl<W> SimpleSender<W, CodecFactoryType, CompressionFactoryType>
where
    W: BaseWriter,
{
    pub fn new_simple(address: Address, writer: W) -> Self {
        Self {
            address,
            writer,
            codec: create_codec(),
            compression: create_compression(),
        }
    }
}
