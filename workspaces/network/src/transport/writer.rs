use defmt::trace;

use crate::error::NetworkError;
use crate::packet_builder::PacketBuilder;
use crate::transport::TransportSender;
use crate::Address;

use codec::Codec;
use physical_layer::BaseWriter;
use sequence_number::SequenceNumber;

#[cfg(feature = "packet-32")]
const SEQUENCE_NUMBER_SIZE: u8 = 8;
#[cfg(feature = "packet-64")]
const SEQUENCE_NUMBER_SIZE: u8 = 32;

pub struct TransportWriter<'a, W, C, P> {
    address: Address,
    sequence_number: SequenceNumber<SEQUENCE_NUMBER_SIZE>,
    resend: u8,

    compression: &'a P,
    codec: &'a C,
    writer: &'a mut W,
}

impl<'a, W, C, P> TransportWriter<'a, W, C, P>
where
    W: BaseWriter,
    C: Codec,
    P: Codec,
{
    pub fn new(
        address: Address,
        resend: u8,
        codec: &'a C,
        compression: &'a P,
        writer: &'a mut W,
    ) -> Self {
        Self {
            address,
            sequence_number: SequenceNumber::new(0),
            resend,
            compression,
            codec,
            writer,
        }
    }
}

impl<'a, W, C, P> TransportSender for TransportWriter<'a, W, C, P>
where
    W: BaseWriter,
    C: Codec,
    P: Codec,
{
    async fn send_bytes(&mut self, payload: &[u8]) -> Result<usize, NetworkError> {
        let mut sent_bytes = 0usize;

        let packet_builder = PacketBuilder::new(
            &self.address,
            &mut self.sequence_number,
            self.compression
                .encode(payload)
                .map_err(NetworkError::CodecError)?,
        );

        for packet in packet_builder {
            trace!("Sending packet = {:?}", packet);
            let data = packet.to_be_bytes();
            for _ in 0..self.resend {
                // TODO don't re-encode the same data multiple times
                // When encoder raise an error we should just stop, as the same data won't be sent at all
                let encoded_data = self.codec.encode(&data).map_err(NetworkError::CodecError)?;
                // FIXME we compute same packet multiple times in retransmision
                sent_bytes += self
                    .writer
                    .write_bytes_iterator(encoded_data)
                    .await
                    .map_err(NetworkError::SenderWriterError)?
            }
        }

        Ok(sent_bytes)
    }
}
