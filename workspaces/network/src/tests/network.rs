use super::io::*;
use crate::transport::reader::TransportReader;
use crate::transport::writer::TransportWriter;
use crate::transport::{TransportReceiver, TransportSender};
use codec::Codec;

pub struct ReaderFactory;

impl ReaderFactory {
    pub fn new() -> Self {
        todo!()
    }

    pub fn create_reader<Cod, Com>(&self) -> TransportReader<'_, DummyManchesterReader, Cod, Com>
    where
        Cod: Codec + Default,
        Com: Codec + Default,
    {
        todo!()
    }
}

pub struct WriterFactory;

impl WriterFactory {
    pub fn new() -> Self {
        todo!()
    }

    pub fn create_writer<Cod, Com>(&self) -> TransportWriter<'_, DummyManchesterWriter, Cod, Com>
    where
        Cod: Codec + Default,
        Com: Codec + Default,
    {
        todo!()
    }
}
