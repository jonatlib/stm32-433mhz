#![no_std]
#![no_main]
#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::cell::RefCell;
use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

mod hardware;
mod payload;
mod transport;

use hardware::io::RadioReceiverPin;
use hardware::io::RadioSenderPin;
use hardware::{Hardware, HardwareSetup};
use network::transport::{TransportReceiver, TransportSender};
use network::Address;

static HARDWARE: StaticCell<Hardware> = StaticCell::new();
static RF_INPUT_PIN: StaticCell<RefCell<ExtiInput<RadioReceiverPin>>> = StaticCell::new();
static RF_OUTPUT_PIN: StaticCell<RefCell<Output<RadioSenderPin>>> = StaticCell::new();

#[embassy_executor::task]
async fn read_task(mut simple_receiver: transport::ReceiverFactory<'static>) {
    let mut transport = simple_receiver.create_transport();

    loop {
        let data: Result<payload::SensorPayload, _> = transport.receive_struct().await;
        info!("---------------------------------------------------");
        info!("Read data = {:?}", data);
        info!("---------------------------------------------------");

        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let hardware: &'static mut Hardware = HARDWARE.init(Hardware::setup_hardware(None));

    ///////////////////
    // Init reader

    let receiver_address = Address::new(0x01, 0x0f);
    let simple_receiver =
        transport::create_transport_receiver(hardware, &RF_INPUT_PIN, receiver_address);
    // spawner.spawn(read_task(simple_receiver)).unwrap();

    ///////////////////
    // Init sender

    let sender_address = Address::new(0x0f, 0x01);
    let mut simple_sender =
        transport::create_transport_sender(hardware, &RF_OUTPUT_PIN, sender_address);
    let mut transport = simple_sender.create_transport();

    // Main loop
    Timer::after(Duration::from_millis(1000)).await;

    let data = payload::SensorPayload {
        timestamp: 123,

        temperature_1: 1.1,
        temperature_2: 2.3,
        humidity: 30,
    };
    loop {
        let _ = transport.send_struct(&data).await;
        Timer::after(Duration::from_millis(5000)).await;
    }
}
