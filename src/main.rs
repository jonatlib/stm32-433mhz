#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use bit_io::{PinReader, PinWriter, SyncReader, SyncSequence, SyncWriter};
use hardware::HardwareSetup;

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use bit_io::reader::ReaderTiming;
use bit_io::writer::WriterTiming;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::PC0;
use embassy_stm32::rcc::ClockSrc;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use network::simple::receiver::SimpleReceiver;
use network::simple::sender::SimpleSender;
use network::transport::{TransportReceiver, TransportSender};
use network::Address;

mod hardware;
mod transport;

#[embassy_executor::task]
async fn read_task(mut simple_receiver: transport::ReceiverFactory<'static>) {
    let mut transport = simple_receiver.create_transport();

    loop {
        let mut data = [0u8; 16];
        let read_size = transport.receive_bytes(&mut data).await;

        info!("---------------------------------------------------");
        info!(
            "Read bytes = {:#04x}, size = {:?}",
            &data[..read_size.as_ref().copied().unwrap_or(0)],
            read_size
        );
        info!("---------------------------------------------------");

        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut hardware = hardware::Hardware::setup_hardware(None);

    let sender_address = Address::new(0x0f, 0x01);
    let mut simple_sender = transport::create_transport_sender(&mut hardware, sender_address);
    let mut transport = simple_sender.create_transport();

    ///////////////////
    // Init reader

    let receiver_address = Address::new(0x01, 0x0f);
    let simple_receiver = transport::create_transport_receiver(&mut hardware, receiver_address);
    spawner.spawn(read_task(simple_receiver)).unwrap();

    ///////////////////

    Timer::after(Duration::from_millis(1000)).await;
    loop {
        let _ = transport
            .send_bytes(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])
            .await;
        Timer::after(Duration::from_millis(5000)).await;
    }
}
