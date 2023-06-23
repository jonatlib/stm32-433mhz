use super::io::*;
use crate::transport::reader::TransportReader;
use crate::transport::writer::TransportWriter;
use crate::transport::{TransportReceiver, TransportSender};
use crate::Address;
use codec::Codec;

pub struct ReaderFactory<Cod, Com> {
    codec: Cod,
    compression: Com,
    reader: DummyManchesterReader,
}

impl<Cod, Com> ReaderFactory<Cod, Com>
where
    Cod: Codec + Default,
    Com: Codec + Default,
{
    pub fn new(reader: DummyManchesterReader) -> Self {
        Self {
            codec: Cod::default(),
            compression: Com::default(),
            reader,
        }
    }

    pub fn create_reader(&mut self) -> TransportReader<'_, DummyManchesterReader, Cod, Com> {
        TransportReader::new(
            Address::new(0x03, 0x08),
            &self.codec,
            &self.compression,
            &mut self.reader,
        )
    }
}

pub struct WriterFactory<Cod, Com> {
    codec: Cod,
    compression: Com,
    writer: DummyManchesterWriter,
}

impl<Cod, Com> WriterFactory<Cod, Com>
where
    Cod: Codec + Default,
    Com: Codec + Default,
{
    pub fn new(writer: DummyManchesterWriter) -> Self {
        Self {
            codec: Cod::default(),
            compression: Com::default(),
            writer,
        }
    }

    pub fn create_writer(&mut self) -> TransportWriter<'_, DummyManchesterWriter, Cod, Com> {
        TransportWriter::new(
            Address::new(0x08, 0x03),
            3,
            &self.codec,
            &self.compression,
            &mut self.writer,
        )
    }
}
