#![no_std]
#![no_main]
#![deny(unsafe_code)]

use defmt::println;
use defmt_rtt as _;
use fugit::RateExtU32;
use nb::block;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::adc::Adc;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::timer::Timer;

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split();
    let io0 = gpioa.pa0.into_pull_up_input(&mut gpioa.crl);

    let mut io1 = gpioa.pa1.into_analog(&mut gpioa.crl);
    let clocks = rcc.cfgr.adcclk(2u32.MHz()).freeze(&mut flash.acr);
    let mut adc = Adc::adc1(dp.ADC1, clocks);
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(1.Hz()).unwrap();
    loop {
        let sig: u16 = adc.read(&mut io1).unwrap();
        if io0.is_low() {
            println!("sensor low");
        }
        println!("input: {}", sig);
        block!(timer.wait()).unwrap();
    }
}
