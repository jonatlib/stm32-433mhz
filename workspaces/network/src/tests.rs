use embedded_hal::serial::Write;
use std::io::{self, Write as _};

pub struct StdoutSerial;

impl Write<u8> for StdoutSerial {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), ()> {
        io::stdout().write(&[word]).unwrap();
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), ()> {
        io::stdout().flush().unwrap();
        Ok(())
    }
}

pub fn init_logging_stdout() {
    let serial = StdoutSerial;
    defmt_serial::defmt_serial(serial);
}
