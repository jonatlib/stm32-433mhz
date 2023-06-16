use codec::CodecError;

#[derive(Debug)]
pub enum NetworkError {
    SenderEncodingError(postcard::Error),
    ReceiverEncodingError(postcard::Error),

    SenderWriterError(physical_layer::error::WriterError),
    ReceiverReaderError(physical_layer::error::ReadError),

    DataConstructingError(DataConstructionError),
    CodecError(CodecError),
}

#[derive(Debug)]
pub enum DataConstructionError {
    FullWindow,
}
