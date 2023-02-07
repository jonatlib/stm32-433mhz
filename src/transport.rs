use crate::hardware::{io, HardwareSetup};

use bit_io::reader::ReaderTiming;
use bit_io::writer::WriterTiming;
use bit_io::{PinReader, PinWriter, SyncReader, SyncSequence, SyncWriter};

use codec::Identity;

use network::simple::sender::SimpleSender;

use network::simple::receiver::SimpleReceiver;
use network::Address;

fn get_sync_sequence() -> SyncSequence {
    SyncSequence::default()
}

fn get_writer_timing() -> WriterTiming {
    WriterTiming::default()
}

fn get_reader_timing() -> ReaderTiming {
    (&get_writer_timing()).into()
}

fn create_codec() -> Identity {
    Identity::default()
}

pub type SenderFactory<'a> =
    SimpleSender<SyncWriter<PinWriter<'a, io::RadioSenderPin, false>>, Identity>;
pub type ReceiverFactory<'a> =
    SimpleReceiver<SyncReader<PinReader<'a, io::RadioReceiverPin, false>>, Identity>;

pub fn create_transport_sender(hw: &impl HardwareSetup, address: Address) -> SenderFactory {
    let output = hw.create_radio_sending_output();

    let pin_writer = PinWriter::<_, false>::new(get_writer_timing(), output)
        .expect("Could not create PinWriter");
    let sync_writer = SyncWriter::new(pin_writer, get_sync_sequence());

    SimpleSender::new(address, sync_writer, create_codec())
}

pub fn create_transport_receiver(hw: &impl HardwareSetup, address: Address) -> ReceiverFactory {
    let input = hw.create_radio_receiving_input();

    let pin_reader =
        PinReader::<_, false>::new(get_reader_timing(), input).expect("Could not create PinReader");
    // 4-bytes to send single packet of 32bits
    let sync_reader = SyncReader::new(pin_reader, get_sync_sequence(), 4);

    SimpleReceiver::new(address, sync_reader, create_codec())
}
