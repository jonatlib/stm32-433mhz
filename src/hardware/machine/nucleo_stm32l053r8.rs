use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::EXTI0;
use embassy_stm32::rcc::ClockSrc;
use embassy_stm32::{Config, Peripherals};

use crate::hardware::io::{RadioReceiverPin, RadioSenderPin};
use crate::hardware::HardwareSetup;

pub struct Hardware {
    peripherals: Peripherals,
}

impl HardwareSetup for Hardware {
    fn setup_hardware(input_config: Option<Config>) -> Self {
        let mut config = input_config.unwrap_or_default();

        // Enable higher clock source
        config.rcc.mux = ClockSrc::HSI16;

        Self {
            peripherals: embassy_stm32::init(config),
        }
    }

    fn get_mut_peripherals(&mut self) -> &mut Peripherals {
        &mut self.peripherals
    }

    fn get_peripherals(&mut self) -> &Peripherals {
        &self.peripherals
    }

    fn create_radio_receiving_input(&self) -> ExtiInput<RadioReceiverPin> {
        unsafe {
            let receiving_input = Input::new(io::RadioReceiverPin::steal(), Pull::None);
            ExtiInput::new(receiving_input, EXTI0::steal())
        }
    }

    fn create_radio_sending_output(&self) -> Output<RadioSenderPin> {
        unsafe { Output::new(io::RadioSenderPin::steal(), Level::Low, Speed::Low) }
    }
}

pub mod io {
    use embassy_stm32::peripherals::{PA5, PC0};

    pub type RadioReceiverPin = PC0;
    pub type RadioSenderPin = PA5;
}