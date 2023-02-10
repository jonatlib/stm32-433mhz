use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::interrupt;
use embassy_stm32::peripherals::EXTI0;
use embassy_stm32::rcc::ClockSrc;
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, Peripherals};

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

    fn create_radio_receiving_input(&self) -> ExtiInput<io::RadioReceiverPin> {
        // FIXME try to do it without unsafe - we are using static allocation in main now
        unsafe {
            let receiving_input = Input::new(io::RadioReceiverPin::steal(), Pull::None);
            ExtiInput::new(receiving_input, EXTI0::steal())
        }
    }

    fn create_radio_sending_output(&self) -> Output<io::RadioSenderPin> {
        unsafe { Output::new(io::RadioSenderPin::steal(), Level::Low, Speed::Low) }
    }

    fn create_i2c(&self) -> io::I2c {
        let peripheral = unsafe { io::I2C1::steal() };
        let sda = unsafe { io::I2C1_SDA::steal() };
        let scl = unsafe { io::I2C1_SCL::steal() };

        let irq = interrupt::take!(I2C1);

        io::I2c::new(
            peripheral,
            scl,
            sda,
            irq,
            NoDma,
            NoDma,
            Hertz(100_000),
            Default::default(),
        )
    }
}

#[allow(non_camel_case_types)]
pub mod io {
    use embassy_stm32::dma::NoDma;
    use embassy_stm32::i2c::I2c as HWI2c;
    use embassy_stm32::peripherals::{I2C1 as Instance, PA5, PB6, PB7, PC0};

    pub type RadioReceiverPin = PC0;
    pub type RadioSenderPin = PA5;

    pub type I2C1 = Instance;
    pub type I2C1_SCL = PB6;
    pub type I2C1_SDA = PB7;
    pub type I2c<'a> = HWI2c<'a, I2C1, NoDma, NoDma>;
}
