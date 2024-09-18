#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt::println;
use defmt_rtt as _;
use fugit::ExtU32;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::{GpioExt, IOPinSpeed, OutputSpeed};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::SysTimerExt;

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
    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();
    let mut timer = cp.SYST.delay(&clocks);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_speed(&mut gpioc.crh, IOPinSpeed::Mhz2);
    let mut light = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    light.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz2);

    loop {
        timer.delay(1.secs());
        println!("blink");
        light.toggle();
        led.toggle();
    }
}
