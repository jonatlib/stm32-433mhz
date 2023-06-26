use crate::transport::reader::TransportReader;
use crate::transport::writer::TransportWriter;
use crate::transport::{TransportReceiver, TransportSender};
use codec::{Codec, CodecSize, Identity};

use crate::tests::network::{ReaderFactory, WriterFactory};
use async_std::task::block_on;
use async_std_test::async_test;
use codec::lzss::LzssCompression;
use std::future::Future;
use std::vec::Vec;

pub mod io;
pub mod network;

// use embedded_hal::serial::Write;
// use std::io::{self, Write as _};
//
// pub struct StdoutSerial;
//
// impl Write<u8> for StdoutSerial {
//     type Error = ();
//
//     fn write(&mut self, word: u8) -> nb::Result<(), ()> {
//         io::stdout().write(&[word]).unwrap();
//         Ok(())
//     }
//
//     fn flush(&mut self) -> nb::Result<(), ()> {
//         io::stdout().flush().unwrap();
//         Ok(())
//     }
// }

pub fn init_logging_stdout() {
    // let serial = StdoutSerial;
    // defmt_serial::defmt_serial(serial);

    let _ = env_logger::builder()
        .filter_module("async_io", log::LevelFilter::Off)
        .filter_module("polling", log::LevelFilter::Off)
        .is_test(true)
        .try_init();
}

pub fn test_network<'a, Result, Callback, Fut, Cod, Com>(callback: Callback) -> Result
where
    for<'b> Callback: FnOnce(ReaderFactory<Cod, Com>, WriterFactory<Cod, Com>) -> Fut,
    Fut: Future<Output = Result> + 'a,
    Cod: Codec + Default + 'a,
    Com: Codec + Default + 'a,
{
    let (reader, writer) = io::prepare_io();
    let mut transport_reader_factory = network::ReaderFactory::new(reader);
    let mut transport_writer_factory = network::WriterFactory::new(writer);

    block_on(callback(transport_reader_factory, transport_writer_factory))
}

pub fn test_configuration<Cod, Com>()
where
    Cod: Codec + Default,
    Com: Codec + Default,
{
    test_network(
        // FIXME how to use `Cod` here?
        |mut reader_factory: ReaderFactory<Identity, Com>, mut writer_factory| async move {
            let mut reader = reader_factory.create_reader();
            let mut writer = writer_factory.create_writer();
            let payload = vec![0x01u8, 0x02, 0x03, 0x04, 0xff, 0xfe, 0xfd, 0xfc, 0xaa];

            let wrote_bytes = writer
                .send_bytes(&payload[..])
                .await
                .expect("Can't send data");
            // FIXME assert for writing won't work

            let mut read_buffer = [0x00u8; 9];
            let read_bytes = reader
                .receive_bytes(&mut read_buffer)
                .await
                .expect("Can't receive data");

            //TODO read bytes assert
            println!("Read = {:?}", read_buffer);
            assert_eq!(payload, Vec::from(read_buffer));
        },
    );
}

#[test]
fn test_full_receive_transmit_identity() {
    test_configuration::<Identity, Identity>();
}

#[test]
fn test_full_receive_transmit_lzmo_compression() {
    test_configuration::<Identity, LzssCompression>();
}
