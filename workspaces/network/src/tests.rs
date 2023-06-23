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
    let mut transport_reader_factory = network::ReaderFactory::new(reader);
    let mut transport_writer_factory = network::WriterFactory::new(writer);

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
            }
        },
    )
    .await;
    Ok(())
}
