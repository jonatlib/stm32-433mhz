use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_stm32::{Config, Peripherals};

pub mod machine;

pub use machine::io;
pub use machine::Hardware;

pub trait HardwareSetup {
    fn setup_hardware(input_config: Option<Config>) -> Self;

    fn get_mut_peripherals(&mut self) -> &mut Peripherals;
    fn get_peripherals(&mut self) -> &Peripherals;

    fn create_radio_receiving_input(&mut self) -> ExtiInput<io::RadioReceiverPin>;
    fn create_radio_sending_output(&mut self) -> Output<io::RadioSenderPin>;
}
