use std::sync::MutexGuard;
use std::vec::Vec;

use physical_layer::error::{ReadError, WriterError};
use physical_layer::{BaseReader, BaseWriter};

use async_std::task::sleep;

enum ChannelData {
    Data(bool),
    Stop,
}

type Channel = std::collections::VecDeque<ChannelData>;
type SharedChannel = std::sync::Arc<std::sync::Mutex<Channel>>;

pub fn prepare_io() -> (DummyManchesterReader, DummyManchesterWriter) {
    let shared_channel = std::sync::Arc::new(std::sync::Mutex::new(Channel::new()));

    (
        DummyManchesterReader::new(shared_channel.clone()),
        DummyManchesterWriter::new(shared_channel.clone()),
    )
}

pub struct DummyManchesterReader(SharedChannel);

impl DummyManchesterReader {
    fn new(channel: SharedChannel) -> Self {
        Self(channel)
    }
}

impl BaseReader for DummyManchesterReader {
    async fn read_bytes_buffer(&mut self, buffer: &mut [u8]) -> Result<usize, ReadError> {
        let mut decoder = manchester::DecoderBool::new(manchester::BitOrder::LittleEndian);
        for element in buffer.iter_mut() {
            let mut nothing_received = 0u8;
            loop {
                let mut guard: MutexGuard<Channel> = self.0.lock().unwrap();
                let received_data = guard.pop_front();
                std::mem::drop(guard);

                if let Some(channel_data) = received_data {
                    nothing_received = 0;
                    match channel_data {
                        ChannelData::Data(bit) => {
                            if let Some(byte) = decoder.next(bit) {
                                *element = byte;
                                break;
                            }
                        }

                        ChannelData::Stop => return Ok(buffer.len()),
                    };
                } else {
                    nothing_received += 1;
                    sleep(std::time::Duration::from_millis(100)).await;
                    if nothing_received > 10 {
                        return Err(ReadError::TimeoutError);
                    }
                }
            }
        }
        // FIXME this is wrong
        Ok(buffer.len())
    }
}

pub struct DummyManchesterWriter(SharedChannel);

impl DummyManchesterWriter {
    fn new(channel: SharedChannel) -> Self {
        Self(channel)
    }
}

impl BaseWriter for DummyManchesterWriter {
    async fn write_bytes_buffer(&mut self, buffer: &[u8]) -> Result<usize, WriterError> {
        self.write_bytes_iterator(buffer.iter().copied()).await?;
        Ok(buffer.len())
    }

    async fn write_bytes_iterator<I: Iterator<Item = u8>>(
        &mut self,
        data: I,
    ) -> Result<usize, WriterError> {
        let mancheser_encoder =
            manchester::EncoderBoolIterator::new(data, manchester::BitOrder::LittleEndian);

        for bit in mancheser_encoder {
            let mut guard: MutexGuard<Channel> = self.0.lock().unwrap();
            guard.push_back(ChannelData::Data(bit));

            sleep(std::time::Duration::from_millis(1)).await;
        }

        let mut guard: MutexGuard<Channel> = self.0.lock().unwrap();
        guard.push_back(ChannelData::Stop);

        Ok(1) // FIXME
    }
}
