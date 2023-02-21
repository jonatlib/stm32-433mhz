use defmt::trace;

use crate::error::NetworkError;
use crate::packet_builder::PacketBuilder;
use crate::transport::TransportSender;
use crate::Address;

use bit_io::BaseWriter;
use codec::Codec;
use sequence_number::SequenceNumber;

pub struct TransportWriter<'a, W, C> {
    address: Address,
    sequence_number: SequenceNumber<8>,
    resend: u8,

    codec: &'a C,
    writer: &'a mut W,
}

impl<'a, W, C> TransportWriter<'a, W, C>
where
    W: BaseWriter,
    C: Codec,
{
    pub fn new(address: Address, resend: u8, codec: &'a C, writer: &'a mut W) -> Self {
        Self {
            address,
            sequence_number: SequenceNumber::new(0),
            resend,
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

        let packet_builder =
            PacketBuilder::new(&self.address, &mut self.sequence_number, payload.iter());

        for packet in packet_builder {
            trace!("Sending packet = {:?}", packet);
            let data = Into::<u32>::into(packet).to_be_bytes();
            for _ in 0..self.resend {
                // TODO don't re-encode the same data multiple times
                // When encoder raise an error we should just stop, as the same data won't be sent at all
                let encoded_data = self
                    .codec
                    .encode(&data)
                    .map_err(|e| NetworkError::CodecError(e))?;
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
