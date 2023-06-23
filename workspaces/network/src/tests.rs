use crate::transport::reader::TransportReader;
use crate::transport::writer::TransportWriter;
use crate::transport::{TransportReceiver, TransportSender};
use codec::{Codec, Identity};

use async_std_test::async_test;
use std::future::Future;

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

pub async fn test_network<'a, Result, Callback, Fut, Cod, Com>(callback: Callback) -> Result
where
    for<'b> Callback: FnOnce(
            &mut TransportReader<'b, io::DummyManchesterReader, Cod, Com>,
            &mut TransportWriter<'b, io::DummyManchesterWriter, Cod, Com>,
        ) -> Fut
        + 'b,
    Fut: Future<Output = Result> + 'a,
    Cod: Codec + Default + 'a,
    Com: Codec + Default + 'a,
{
    let (reader, writer) = io::prepare_io();
    let transport_reader_factory = network::ReaderFactory::new();
    let transport_writer_factory = network::WriterFactory::new();

    let mut reader = transport_reader_factory.create_reader();
    let mut writer = transport_writer_factory.create_writer();
    callback(&mut reader, &mut writer).await
}

#[async_test]
async fn test_full_receive_transmit() -> std::io::Result<()> {
    test_network(
        |reader: &mut TransportReader<'_, io::DummyManchesterReader, Identity, Identity>,
         writer| {
            async move {
                todo!();
            }
        },
    )
    .await;
    Ok(())
}
