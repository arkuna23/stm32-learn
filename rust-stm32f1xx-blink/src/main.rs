#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt::println;
use defmt_rtt as _;
use fugit::ExtU32;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    gpio::{IOPinSpeed, OutputSpeed},
    pac,
    prelude::*,
};

macro_rules! setup_pin {
    ($gpio:expr, $pin:expr, $speed:expr) => {{
        let mut pin = $pin.into_push_pull_output(&mut $gpio.crl);
        pin.set_high();
        pin.set_speed(&mut $gpio.crl, $speed);
        pin.erase()
    }};
}

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioc = dp.GPIOC.split();
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut gpioa = dp.GPIOA.split();
    let mut lights = [
        setup_pin!(gpioa, gpioa.pa0, IOPinSpeed::Mhz2),
        setup_pin!(gpioa, gpioa.pa1, IOPinSpeed::Mhz2),
        setup_pin!(gpioa, gpioa.pa2, IOPinSpeed::Mhz2),
        setup_pin!(gpioa, gpioa.pa3, IOPinSpeed::Mhz2),
    ];
    let mut timer = cp.SYST.delay(&clocks);

    loop {
        for ele in lights.as_mut() {
            println!("blink");
            led.toggle();
            ele.set_low();
            timer.delay(500_u32.millis());
            ele.set_high();
        }
    }
}
