#![no_std]
#![no_main]

use defmt::println;
use defmt_rtt as _;
use display::Number;
use fugit::RateExtU32;
use nb::block;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::pac::{self, SYST};
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::{Clocks, RccExt};
use stm32f1xx_hal::timer::{self, SysCounterHz};
use tube::Tube;

mod display;
mod tube;

pub struct Timer {
    timer: SysCounterHz,
    count: u64,
}

impl Timer {
    pub fn new(hz: u32, syst: SYST, clocks: &Clocks) -> Self {
        let mut timer = timer::Timer::syst(syst, clocks).counter_hz();
        timer.start(hz.Hz()).unwrap();
        Self { timer, count: 0 }
    }

    pub fn delay(&mut self) {
        block!(self.timer.wait()).unwrap();
        self.count += 1;
    }

    #[inline(always)]
    pub fn count(&self) -> u64 {
        self.count
    }
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

    let mut timer = Timer::new(24, cp.SYST, &clocks);
    let mut tube = Tube::new(dp.GPIOA, dp.GPIOB);

    let mut secs = Number(0);
    println!("start loop");
    tube.set_tube(secs);
    loop {
        if timer.count() % 24 == 0 {
            secs.0 += 1;
            tube.set_tube(secs);
        }
        tube.show(&mut timer);
    }
}
