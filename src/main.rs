#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use bit_io::{PinReader, PinWriter, Reader, SyncReader, SyncSequence, SyncWriter, Writer};

use defmt::info;
use {defmt_rtt as _, panic_probe as _};

use bit_io::reader::ReaderTiming;
use bit_io::writer::WriterTiming;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::peripherals::PC0;
use embassy_stm32::rcc::{ClockSrc, PLLSource};
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn read_task(button: ExtiInput<'static, PC0>, sync: SyncSequence, timing: ReaderTiming) {
    let mut reader = PinReader::<_, false>::new(timing, button).expect("Could not create bit_io");
    let mut reader = SyncReader::new(reader, sync, 4);

    loop {
        // let byte = reader.read_byte().await;
        // info!("---------------------------------------------------");
        // info!("Read byte = {:#04x}", byte);
        // info!("---------------------------------------------------");

        let mut data: [u8; 4] = [0; 4];
        let read_size = reader.read_bytes(&mut data).await;
        info!("---------------------------------------------------");
        info!("Read bytes = {:#04x}, size = {}", data, read_size);
        info!("---------------------------------------------------");

        Timer::after(Duration::from_millis(2000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.mux = ClockSrc::HSI16;
    let p = embassy_stm32::init(config);

    let sync = SyncSequence::default();
    let writer_timing = WriterTiming::default();

    let led = Output::new(p.PA5, Level::Low, Speed::Low);
    let mut writer =
        PinWriter::<_, false>::new(writer_timing, led).expect("Could not create bit_io");
    let mut writer = SyncWriter::new(writer, sync.clone());

    // Configure the button pin and obtain handler.
    // On the Nucleo F091RC there is a button connected to pin PC13.
    let writer_pin = Input::new(p.PC0, Pull::None);
    let button = ExtiInput::new(writer_pin, p.EXTI0);
    spawner
        .spawn(read_task(
            button,
            sync,
            ReaderTiming::from(writer.get_timing()),
        ))
        .unwrap();

    let data = [0xf0u8, 0x0f, 0xef, 0xba];
    loop {
        // writer.write_byte(0xf0).await.unwrap();
        writer.write_bytes(&data).await;
        Timer::after(Duration::from_millis(2000)).await;
    }
}
