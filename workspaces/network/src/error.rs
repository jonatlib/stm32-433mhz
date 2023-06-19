use codec::CodecError;
use defmt::Format;

#[derive(Debug, Format)]
pub enum NetworkError {
    SenderEncodingError(postcard::Error),
    ReceiverEncodingError(postcard::Error),

    SenderWriterError(physical_layer::error::WriterError),
    ReceiverReaderError(physical_layer::error::ReadError),

    DataConstructingError(DataConstructionError),
    CodecError(CodecError),
}

#[derive(Debug, Format)]
pub enum DataConstructionError {
    FullWindow,
    WrongStreamId,
}
