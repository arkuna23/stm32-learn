#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::pac::{self};
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;

use stm32f1xx_hal::gpio::{ gpioa::Parts as PartsA, gpiob::Parts as PartsB };

mod tube;



#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);

    loop {
    }
}
