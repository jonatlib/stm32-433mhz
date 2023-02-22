use crate::hardware::{io, HardwareSetup};
use embassy_time::Duration;

use bit_io::reader::ReaderTiming;
use bit_io::writer::WriterTiming;
use bit_io::{PinReader, PinWriter, SyncReader, SyncSequence, SyncWriter};

use codec::lzss::LzssCompression;
use codec::reed_solomon::ReedSolomon;

use network::simple::sender::SimpleSender;

use network::simple::receiver::SimpleReceiver;
use network::Address;

fn get_sync_sequence() -> SyncSequence {
    SyncSequence::new_simple(Duration::from_micros(1500), 4, 0b1011)
}

fn get_writer_timing() -> WriterTiming {
    WriterTiming::new(
        Duration::from_micros(500),
        Duration::from_micros(800),
        Duration::from_micros(300),
        None,
    )
}

fn get_reader_timing() -> ReaderTiming {
    ReaderTiming::new(
        Duration::from_micros(450),
        Duration::from_micros(750),
        Duration::from_micros(400),
        Duration::from_micros(1000),
    )
}

type CodecType = ReedSolomon<4, 4>;
type CompressionType = LzssCompression;

fn create_codec() -> CodecType {
    CodecType::default()
}
fn create_compression() -> CompressionType {
    CompressionType::default()
}

pub type SenderFactory<'a> =
    SimpleSender<SyncWriter<PinWriter<'a, io::RadioSenderPin, false>>, CodecType, CompressionType>;
pub type ReceiverFactory<'a> = SimpleReceiver<
    SyncReader<PinReader<'a, io::RadioReceiverPin, false>>,
    CodecType,
    CompressionType,
>;

pub fn create_transport_sender(hw: &impl HardwareSetup, address: Address) -> SenderFactory {
    let output = hw.create_radio_sending_output();

    let pin_writer = PinWriter::<_, false>::new(get_writer_timing(), output)
        .expect("Could not create PinWriter");
    let sync_writer = SyncWriter::new(pin_writer, get_sync_sequence());

    SimpleSender::new(address, sync_writer, create_codec(), create_compression())
}

pub fn create_transport_receiver(hw: &impl HardwareSetup, address: Address) -> ReceiverFactory {
    let input = hw.create_radio_receiving_input();

    let pin_reader =
        PinReader::<_, false>::new(get_reader_timing(), input).expect("Could not create PinReader");
    // 4-bytes to send single packet of 32bits
    let sync_reader = SyncReader::new(pin_reader, get_sync_sequence(), 4);

    SimpleReceiver::new(address, sync_reader, create_codec(), create_compression())
}
