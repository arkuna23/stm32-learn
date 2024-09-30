#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::{Enable, RccExt, Reset};

fn enable_iopc() {
    use pac::GPIOC;
    let rcc = unsafe { &(*pac::RCC::ptr()) };
    GPIOC::enable(rcc);
    GPIOC::reset(rcc);
}

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let _clocks = rcc.cfgr.freeze(&mut flash.acr);
    enable_iopc();
    dp.GPIOC
        .crh
        .modify(|r, w| unsafe { w.bits(r.bits() | 0b0010 << (4 * (13 - 8))) });

    dp.GPIOC
        .odr
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << 13)) });
    loop {}
}
