use crate::hardware::{io, HardwareSetup};
use core::cell::RefCell;
use embassy_stm32::exti::ExtiInput;
use embassy_time::Duration;
use static_cell::StaticCell;

use physical_layer::pwm::ReaderTiming;
use physical_layer::pwm::WriterTiming;
use physical_layer::pwm::{
    PinPwmReader, PinPwmWriter, PwmSyncMarkerReader, SyncPwmWriter, SyncSequence,
};

use crate::hardware::io::RadioReceiverPin;

use codec::lzss::LzssCompression;
use codec::reed_solomon::ReedSolomon;
use network::simple::receiver::SimpleReceiver;
use network::simple::sender::SimpleSender;
use network::Address;
use physical_layer::sync::reader::SyncReader;
use physical_layer::utils::SharedExtiPin;

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

// type CodecType = FourToSixBits<10>; // FIXME wtf this is not 4?
type CodecType = ReedSolomon<4, 4>;
// type CodecType = Identity;
// type CodecType = Chain<ReedSolomon<4, 4>, FourToSixBits<20>, 4>; // FIXME FourToSixBits buffer weird

type CompressionType = LzssCompression;
// type CompressionType = Identity;

fn create_codec() -> CodecType {
    CodecType::default()
}
fn create_compression() -> CompressionType {
    CompressionType::default()
}

pub type SenderFactory<'a> = SimpleSender<
    SyncPwmWriter<PinPwmWriter<'a, io::RadioSenderPin, false>>,
    CodecType,
    CompressionType,
>;
pub type ReceiverFactory<'a> = SimpleReceiver<
    SyncReader<
        PinPwmReader<'a, io::RadioReceiverPin, false>,
        PwmSyncMarkerReader<PinPwmReader<'a, io::RadioReceiverPin, false>>,
    >,
    CodecType,
    CompressionType,
>;

pub fn create_transport_sender(hw: &impl HardwareSetup, address: Address) -> SenderFactory {
    let output = hw.create_radio_sending_output();

    let pin_writer = PinPwmWriter::<_, false>::new(get_writer_timing(), output)
        .expect("Could not create PinWriter");
    let sync_writer = SyncPwmWriter::new(pin_writer, get_sync_sequence());

    SimpleSender::new(address, sync_writer, create_codec(), create_compression())
}

pub fn create_transport_receiver(
    hw: &'static impl HardwareSetup,
    input_pin_cell: &'static StaticCell<RefCell<ExtiInput<RadioReceiverPin>>>,
    address: Address,
) -> ReceiverFactory<'static> {
    let input = hw.create_radio_receiving_input();
    let shared_input = SharedExtiPin::new(input, input_pin_cell);

    let pin_sync_reader = PinPwmReader::<_, false>::new(get_reader_timing(), shared_input)
        .expect("Could not create PinReader");
    let pin_data_reader = PinPwmReader::<_, false>::new(get_reader_timing(), shared_input)
        .expect("Could not create PinReader");

    // 4-bytes to send single packet of 32bits
    let sync = PwmSyncMarkerReader::new(pin_sync_reader, get_sync_sequence());
    let sync_reader = SyncReader::new(sync, pin_data_reader, Duration::from_micros(0));

    SimpleReceiver::new(address, sync_reader, create_codec(), create_compression())
}
