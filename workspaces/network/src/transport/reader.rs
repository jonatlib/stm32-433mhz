use crate::error::NetworkError;
use crate::packet::PacketType;
use crate::transport::window::Window;
use crate::transport::TransportReceiver;
use crate::Address;

use codec::{Codec, CodecSize};
use physical_layer::BaseReader;

#[cfg(not(test))]
use defmt::{error, trace};

#[cfg(test)]
use log::{error, trace};

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
    [(); C::get_encode_const_size(8)]: Sized,
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
            // Update: Abowe is correct for Packet32 but not Packet64 - changing to 8
            let mut reader_buffer = [0u8; C::get_encode_const_size(8)];

            // FIXME what about packet size 32/64
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
                error!("Decoding data error = {:?}", decoded_data_result.err());
                continue;
            }
            let decoded_data = decoded_data_result.expect("This cant be error after the if");
            let mut packet_buffer = [0u8; 8]; // One packet is 32bit = 4bytes// Update: Packet64 -> 8
            for (index, byte) in decoded_data.enumerate() {
                packet_buffer[index] = byte;
            }

            // And here is our packet (comment for readability)
            let packet: PacketType = u64::from_be_bytes(packet_buffer).into();
            trace!("Received packet = {:?}", packet);
            if packet.destination_address() != self.address.local_address {
                trace!(
                    "Received packet for different address = {}. Expected = {}",
                    packet.destination_address(),
                    self.address.local_address
                );
                continue;
            }

            // FIXME how long to wait for missing packets?
            // FIXME maybe when received packet outside of sequence numbers?
            // FIXME or do we need some stream id and when it change we strip this stream?

            let window_status = self.window.push_packet(packet)?;
            if let Some(size) = window_status {
                // FIXME what about this with packet64?
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::fmt::Formatter;
    use std::future::Future;

    use crate::tests::init_logging_stdout;
    use codec::Identity;
    use physical_layer::error::ReadError;

    use std::time::Duration;
    use std::vec::Vec;

    use crate::packet::{Packet32, PacketKind};
    use crate::simple::receiver::SimpleReceiver;
    use async_std::future::timeout;
    use async_std_test::async_test;
    use sequence_number::SequenceNumber;

    #[derive(Debug)]
    struct BaseError;

    impl std::error::Error for BaseError {}

    impl std::fmt::Display for BaseError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl From<std::io::Error> for BaseError {
        fn from(value: std::io::Error) -> Self {
            BaseError
        }
    }

    struct DummyReader(VecDeque<u8>, usize);

    impl BaseReader for DummyReader {
        async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
            async_std::task::sleep(Duration::from_millis(500)).await;
            for (index, value) in buffer.iter_mut().enumerate() {
                async_std::task::sleep(Duration::from_millis(10)).await;
                if let Some(v) = self.0.pop_front() {
                    *value = v;
                } else {
                    break;
                }
            }
            Ok(buffer.len().min(self.1))
        }
    }

    struct DummyReceiver {
        address: Address,
        codec: Identity,
        compression: Identity,
        reader: DummyReader,
    }

    impl DummyReceiver {
        fn new(payload: VecDeque<u8>) -> Self {
            let len = payload.len();
            Self {
                address: Address::new(0x01, 0x05),
                codec: Identity::default(),
                compression: Identity::default(),
                reader: DummyReader(payload, len),
            }
        }

        fn create_receiver(&mut self) -> TransportReader<'_, DummyReader, Identity, Identity> {
            TransportReader::new(
                self.address.clone(),
                &self.codec,
                &self.compression,
                &mut self.reader,
            )
        }
    }

    async fn receiver_environment_single_packet_32<'a, C, F>(callback: C) -> std::io::Result<()>
    where
        C: FnOnce(Packet32, DummyReceiver) -> F,
        F: Future<Output = std::io::Result<()>> + 'a,
    {
        init_logging_stdout();
        let original_packet = Packet32::new()
            .with_kind(PacketKind::SelfContained)
            .with_source_address(0x05)
            .with_destination_address(0x01)
            .with_payload(0xabcd)
            .with_payload_used_index(1);

        let mut factory = DummyReceiver::new(
            original_packet
                .clone()
                .to_be_bytes()
                .into_iter()
                .collect::<VecDeque<u8>>(),
        );

        callback(original_packet, factory).await
    }

    async fn receiver_environment_three_packets_32<'a, C, F>(callback: C) -> std::io::Result<()>
    where
        C: FnOnce(Vec<Packet32>, DummyReceiver) -> F,
        F: Future<Output = std::io::Result<()>> + 'a,
    {
        init_logging_stdout();
        let original_packet = Packet32::new()
            .with_kind(PacketKind::SelfContained)
            .with_source_address(0x05)
            .with_destination_address(0x01)
            .with_payload(0x0000)
            .with_payload_used_index(1);

        let packets = vec![
            original_packet
                .with_kind(PacketKind::Start)
                .with_sequence_number(SequenceNumber::new(0))
                .with_payload(0x0102),
            original_packet
                .with_kind(PacketKind::Continue)
                .with_sequence_number(SequenceNumber::new(1))
                .with_payload(0x4567),
            original_packet
                .with_kind(PacketKind::End)
                .with_sequence_number(SequenceNumber::new(2))
                .with_payload(0xabcd),
        ];

        let mut factory = DummyReceiver::new(
            packets
                .clone()
                .into_iter()
                .map(|p| p.to_be_bytes())
                .flatten()
                .into_iter()
                .collect::<VecDeque<u8>>(),
        );

        callback(packets, factory).await
    }

    #[async_test]
    async fn test_receive_single_packet() -> std::io::Result<()> {
        receiver_environment_single_packet_32(|_, mut factory| async move {
            let mut receiver = factory.create_receiver();
            let mut receive_buffer = [0u8; 8];
            let read_size = timeout(
                Duration::from_secs(2),
                receiver.receive_bytes(&mut receive_buffer),
            )
            .await
            .unwrap()
            .unwrap();

            println!("{:#04x?}", receive_buffer);

            assert_eq!(read_size, 2);
            assert_eq!(receive_buffer[0], 0xab);
            assert_eq!(receive_buffer[1], 0xcd);
            assert_eq!(receive_buffer[2], 0x00);

            Ok(())
        })
        .await
    }

    #[async_test]
    async fn test_receive_multiple_packets() -> std::io::Result<()> {
        receiver_environment_three_packets_32(|_, mut factory| async move {
            let mut receiver = factory.create_receiver();
            let mut receive_buffer = [0u8; 8];
            let read_size = timeout(
                Duration::from_secs(2),
                receiver.receive_bytes(&mut receive_buffer),
            )
            .await
            .unwrap()
            .unwrap();

            println!("{:#04x?}", receive_buffer);

            assert_eq!(read_size, 6);
            assert_eq!(receive_buffer[0], 0x01);
            assert_eq!(receive_buffer[1], 0x02);
            assert_eq!(receive_buffer[2], 0x45);
            assert_eq!(receive_buffer[3], 0x67);
            assert_eq!(receive_buffer[4], 0xab);
            assert_eq!(receive_buffer[5], 0xcd);

            Ok(())
        })
        .await
    }
}
