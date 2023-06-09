use defmt::Format;

#[derive(Format, Debug)]
pub enum ReadError {
    TimeoutError,
    ThresholdError,
    OutOfTiming,
}

impl ReadError {
    pub fn is_recoverable(&self) -> bool {
        !matches!(self, ReadError::TimeoutError)
    }
}

#[derive(Format, Debug)]
pub enum WriterError {
    RuntimeError,
}

impl WriterError {
    pub fn is_recoverable(&self) -> bool {
        true
    }
}
