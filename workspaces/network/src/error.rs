pub enum NetworkError {
    SenderEncodingError(postcard::Error),
    ReceiverEncodingError(postcard::Error),

    SenderWriterError(bit_io::error::WriterError),
    ReceiverReaderError(bit_io::error::ReadError),

    DataConstructingError(DataConstructionError),
}

pub enum DataConstructionError {
    FullWindow,
}
