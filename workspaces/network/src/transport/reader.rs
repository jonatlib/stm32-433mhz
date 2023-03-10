use crate::error::NetworkError;
use crate::packet::Packet32;
use crate::transport::window::Window;
use crate::transport::TransportReceiver;
use crate::Address;
use bit_io::BaseReader;
use codec::{Codec, CodecSize};

pub struct TransportReader<'a, R, C, P> {
    address: Address,
    window: Window<8>,

    codec: &'a C,
    compression: &'a P,
    reader: &'a mut R,
}

impl<'a, R, C, P> TransportReader<'a, R, C, P>
where
    R: BaseReader,
    C: Codec,
    P: Codec,
{
    pub fn new(address: Address, codec: &'a C, compression: &'a P, reader: &'a mut R) -> Self {
        Self {
            address,
            window: Window::new(),

            codec,
            compression,
            reader,
        }
    }
}

impl<'a, R, C, P> TransportReceiver for TransportReader<'a, R, C, P>
where
    R: BaseReader,
    C: Codec + ~const CodecSize,
    P: Codec + ~const CodecSize,
    [(); C::get_encode_const_size(4)]: Sized,
{
    async fn receive_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError> {
        self.window.clear();

        loop {
            // FIXME Beware encoded can be longer than 4bytes
            // Maximum received size should be up to 4bytes per packet and up to 8packets
            // so maximum is 32bytes.
            // But in sender we encode and send each packet one by one
            // This way we should be receiving only 4 bytes before encoding.
            // So the number of received bytes is basically how much bytes is needed to encode 4 bytes
            let mut reader_buffer = [0u8; C::get_encode_const_size(4)];

            let read_size = C::get_encode_size(4);
            let received_size = self
                .reader
                .read_bytes_buffer(&mut reader_buffer[..read_size])
                .await
                .map_err(NetworkError::ReceiverReaderError)?;

            // This should be then data worth of one packet only (4 bytes)
            let decoded_data_result = self.codec.decode(&reader_buffer[..received_size]);
            // FIXME what about decode errors?
            //   We can receive packet multiple times even when the first
            //   transmission is broken
            if decoded_data_result.is_err() {
                continue;
            }
            let decoded_data = decoded_data_result.expect("This cant be error after the if");
            let mut packet_buffer = [0u8; 4]; // One packet is 32bit = 4bytes
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
                let mut compressed_buffer = [0u8; 16]; // Packet can hold up to 16bytes
                self.window
                    .write_buffer(&mut compressed_buffer)
                    .expect("This should not happen as push is called just before.");

                let decompressed = self
                    .compression
                    .decode(&compressed_buffer[..size])
                    .map_err(NetworkError::CodecError)?;

                let mut decompress_size = 0usize;
                for (index, byte) in decompressed.enumerate() {
                    buffer[index] = byte;
                    decompress_size += 1;
                }

                return Ok(decompress_size);
            }
        }
    }
}
