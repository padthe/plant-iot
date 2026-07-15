#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, DMA_CH1, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::Timer;
use panic_halt as _;

#[embassy_executor::main]
async fn main(__spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());

    let mut led = Output::new(peripherals.PIN_16, Level::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await;

        led.set_low();
        Timer::after_millis(500).await;
    }
}
