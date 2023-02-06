use crate::error::NetworkError;
use crate::packet::Packet32;
use crate::transport::window::Window;
use crate::transport::TransportReceiver;
use crate::Address;
use bit_io::BaseReader;
use codec::Codec;

pub struct TransportReader<'a, R, C> {
    address: Address,
    window: Window<8>,

    codec: &'a C,
    reader: &'a mut R,
}

impl<'a, R, C> TransportReader<'a, R, C>
where
    R: BaseReader,
    C: Codec,
{
    pub fn new(address: Address, codec: &'a C, reader: &'a mut R) -> Self {
        Self {
            address,
            window: Window::new(),

            codec,
            reader,
        }
    }
}

impl<'a, R, C> TransportReceiver for TransportReader<'a, R, C>
where
    R: BaseReader,
    C: Codec,
{
    async fn receive_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        self.window.clear();

        loop {
            // Maximum received size should be up to 4bytes per packet and up to 8packets
            // so maximum is 32bytes.
            // But in sender we encode and send each packet one by one
            // This way we should be receiving only 4 bytes before encoding.
            // So the number of received bytes is basically how much bytes is needed to encode 4 bytes
            let mut reader_buffer = [0u8; 16]; // TODO how big this buffer should be?
            let read_size = C::get_encode_size(4);
            let received_size = self
                .reader
                .read_bytes_buffer(&mut reader_buffer[..read_size])
                .await
                .map_err(NetworkError::ReceiverReaderError)?;

            // This should be then data worth of one packet only (4 bytes)
            let decoded_data = self.codec.decode(&reader_buffer[..received_size]);
            let mut packet_buffer = [0u8; 4];
            for (index, byte) in decoded_data.enumerate() {
                packet_buffer[index] = byte;
            }

            // And here is our packet (comment for readability)
            let packet: Packet32 = u32::from_be_bytes(packet_buffer).into();
            if packet.destination_address() != self.address.local_address {
                continue;
            }

            // FIXME how long to wait for missing packets?
            // FIXME maybe when received packet outside of sequence numbers?
            // FIXME or do we need some stream id and when it change we strip this stream?

            let window_status = self.window.push_packet(packet)?;
            if let Some(size) = window_status {
                self.window
                    .write_buffer(buffer)
                    .expect("This should not happen as push is called just before.");
                return Ok(size);
            }
        }
    }
}
