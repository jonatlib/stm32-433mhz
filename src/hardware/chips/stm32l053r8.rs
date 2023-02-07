use crate::hardware::HardwareSetup;

pub struct Hardware {}

impl HardwareSetup for Hardware {}

pub mod io {
    use embassy_stm32::exti::ExtiInput;
    use embassy_stm32::peripherals::PC0;

    // TODO pin types, etc

    pub type ReceiverPin = ExtiInput<'static, PC0>;
}
