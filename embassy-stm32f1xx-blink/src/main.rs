#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Level, Output, Speed}, Config};
use embassy_time::Timer;
use heapless::{pool::boxed::Box, Vec};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Config::default());
    let mut outs = Vec::new();

    outs.push(Box::From(Output::new(p.PA0,Level::High,Speed::Low)));
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        led.toggle();
        Timer::after_secs(2).await;
    }
}
