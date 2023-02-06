use defmt::trace;

use crate::error::NetworkError;
use crate::packet_builder::PacketBuilder;
use crate::transport::TransportSender;
use crate::Address;

use bit_io::BaseWriter;
use codec::Codec;

pub struct TransportWriter<'a, W, C> {
    address: Address,

    codec: &'a C,
    writer: &'a mut W,
}

impl<'a, W, C> TransportWriter<'a, W, C>
where
    W: BaseWriter,
    C: Codec,
{
    pub fn new(address: Address, codec: &'a C, writer: &'a mut W) -> Self {
        Self {
            address,
            codec,
            writer,
        }
    }
}

impl<'a, W, C> TransportSender for TransportWriter<'a, W, C>
where
    W: BaseWriter,
    C: Codec,
{
    async fn send_bytes(&mut self, payload: &[u8]) -> Result<usize, NetworkError> {
        let mut sent_bytes = 0usize;
        let packet_builder = PacketBuilder::new(&self.address, 1, payload.iter());
        // TODO support sending each packet multiple times if it get lost
        for packet in packet_builder {
            trace!("Sending packet = {:?}", packet);
            let data = Into::<u32>::into(packet).to_be_bytes();
            let encoded_data = self.codec.encode(&data);
            sent_bytes += self
                .writer
                .write_bytes_iterator(encoded_data)
                .await
                .map_err(NetworkError::SenderWriterError)?
        }

        Ok(sent_bytes)
    }
}
