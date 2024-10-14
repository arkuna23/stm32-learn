#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt_rtt as _;
use fugit::RateExtU32;
use iic_oled_rs::Oled;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::afio::AfioExt;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
// 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        .use_hse(8u32.MHz())
        .sysclk(72u32.MHz())
        .pclk1(36u32.MHz())
        .pclk2(72u32.MHz())
        .freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400u32.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );
    let mut oled = Oled::<64, _>::new(i2c);
    oled.init().unwrap();
    oled.clear().unwrap();

    let mut count = 0;
    loop {
    }
}
