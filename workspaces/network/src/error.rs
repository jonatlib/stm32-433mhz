pub enum NetworkError {
    SenderEncodingError(postcard::Error),

    SenderWriterError(bit_io::error::WriterError),
}
