use crate::hardware::{io, HardwareSetup};

use bit_io::reader::ReaderTiming;
use bit_io::writer::WriterTiming;
use bit_io::{PinWriter, SyncSequence, SyncWriter};

use codec::{Codec, Identity};

use network::simple::sender::SimpleSender;
use network::transport::TransportSender;
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

pub fn create_transport_sender(
    hw: &mut impl HardwareSetup,
    address: Address,
) -> SimpleSender<SyncWriter<PinWriter<io::RadioSenderPin, false>>, Identity> {
    let output = hw.create_radio_sending_output();

    let pin_writer = PinWriter::<_, false>::new(get_writer_timing(), output)
        .expect("Could not create PinWriter");
    let sync_writer = SyncWriter::new(pin_writer, get_sync_sequence());

    SimpleSender::new(address, sync_writer, create_codec())
}
