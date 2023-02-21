use codec::CodecError;
use defmt::Format;

#[derive(Format, Debug)]
pub enum NetworkError {
    SenderEncodingError(postcard::Error),
    ReceiverEncodingError(postcard::Error),

    SenderWriterError(bit_io::error::WriterError),
    ReceiverReaderError(bit_io::error::ReadError),

    DataConstructingError(DataConstructionError),
    CodecError(CodecError),
}

#[derive(Format, Debug)]
pub enum DataConstructionError {
    FullWindow,
}
