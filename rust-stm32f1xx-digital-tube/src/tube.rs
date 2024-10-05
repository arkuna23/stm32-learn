use defmt::println;
use stm32f1xx_hal::{
    gpio::{ErasedPin, GpioExt, IOPinSpeed, Output, OutputSpeed, PinState},
    pac::{GPIOA, GPIOB},
};

use crate::Timer;

pub const SEGMENTS_MASK: u32 = 0b0001_1111_1110_0000;

pub struct Segments {
    pub state: u32,
}
impl Default for Segments {
    fn default() -> Self {
        Self {
            state: SEGMENTS_MASK,
        }
    }
}
impl Segments {
    pub fn init(gpioa: &GPIOA) {
        gpioa.crl.modify(|r, w| {
            let mut crl = r.bits();
            for i in 0..=7 {
                let inuse = (SEGMENTS_MASK >> i) & 1;
                if inuse == 1 {
                    println!("i: {}", i);
                    crl &= !(0b1111 << (i * 4));
                    // 0001: 通用推免输出，最大速度10MHz
                    // 0010: 通用推免输出，最大速度2MHz
                    crl |= 0b0001 << (i * 4);
                }
            }
            unsafe { w.bits(crl) }
        });
        gpioa.crh.modify(|r, w| {
            let mut crh = r.bits();
            for i in 0..=7 {
                println!("i: {}", i);
                let inuse = (SEGMENTS_MASK >> (i + 8)) & 1;
                if inuse == 1 {
                    crh &= !(0b1111 << (i * 4));
                    // 0001: 通用推免输出，最大速度10MHz
                    // 0010: 通用推免输出，最大速度2MHz
                    crh |= 0b0001 << (i * 4);
                }
            }
            println!("crh: {:032b}", crh);
            unsafe { w.bits(crh) }
        });
    }

    /// 改变当前控制的数码管显示状态，需要先改变当前控制的数码管
    pub fn set_gpio(&self, gpioa: &GPIOA) {
        gpioa.odr.modify(|r, w| unsafe {
            // 1代表不显示, 0代表显示
            let bits = (r.bits() | SEGMENTS_MASK) & !self.state;
            w.bits(bits)
        });
    }
}

pub trait SegDisplay {
    /// 数码管显示状态
    fn bits(self) -> u32;

    /// 合并两个数码管显示状态
    #[allow(dead_code)]
    #[inline(always)]
    fn combine(self, other: impl SegDisplay) -> u32
    where
        Self: Sized,
    {
        self.bits() | other.bits()
    }
}

impl SegDisplay for u32 {
    #[inline(always)]
    fn bits(self) -> u32 {
        self
    }
}

pub trait TubeDisplay {
    fn tube_bits(self) -> [impl SegDisplay; 4];
}
impl<T: SegDisplay> TubeDisplay for [T; 4] {
    #[inline(always)]
    fn tube_bits(self) -> [impl SegDisplay; 4] {
        self
    }
}

/// 5461BS-1 4位
/// [文档](http://www.xlitx.com/datasheet/5461BS.pdf)
/// A(11) -> PA7
/// B(7) -> PA6
/// F(10) -> PA5
/// E(1) -> PA8
/// D(2) -> PA9
/// DP(3) -> PA10
/// C(4) -> PA11
/// G(5) -> PA12
pub struct Segment;

impl Segment {
    pub const A: u32 = 1 << 7;
    pub const B: u32 = 1 << 6;
    pub const F: u32 = 1 << 5;
    pub const E: u32 = 1 << 8;
    pub const D: u32 = 1 << 9;
    pub const DP: u32 = 1 << 10;
    pub const C: u32 = 1 << 11;
    pub const G: u32 = 1 << 12;
}

/// 5461BS-1 4位
/// [文档](http://www.xlitx.com/datasheet/5461BS.pdf)
/// DIG1(12) -> PB11
/// DIG2(9) -> PB10
/// DIG3(8) -> PB1
/// DIG4(6) -> PB15
pub struct Tube {
    segments_n: [(ErasedPin<Output>, Segments); 4],
    gpioa: GPIOA,
}

macro_rules! gpio_init_low {
    ($gpio:expr, $pin: expr) => {{
        let mut io = $pin.into_push_pull_output_with_state(&mut $gpio.crl, PinState::Low);
        io.set_speed(&mut $gpio.crl, IOPinSpeed::Mhz2);
        io.erase()
    }};
}

macro_rules! gpio_init_high {
    ($gpio:expr, $pin: expr) => {{
        let mut io = $pin.into_push_pull_output_with_state(&mut $gpio.crh, PinState::Low);
        io.set_speed(&mut $gpio.crh, IOPinSpeed::Mhz2);
        io.erase()
    }};
}

macro_rules! gpio_enable {
    ($GPIO: ident) => {
        use stm32f1xx_hal::rcc::{Enable, Reset};
        let rcc = unsafe { &(*stm32f1xx_hal::pac::RCC::ptr()) };
        $GPIO::enable(rcc);
        $GPIO::reset(rcc);
    };
}

impl Tube {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB) -> Tube {
        static mut INITED: bool = false;

        assert!(!unsafe { INITED }, "Tube has been initialized");
        unsafe {
            INITED = true;
        };
        println!("init digital tube...");
        gpio_enable!(GPIOA);
        Segments::init(&gpioa);
        let mut gpiob = gpiob.split();

        Self {
            segments_n: [
                (gpio_init_high!(gpiob, gpiob.pb11), Segments::default()),
                (gpio_init_high!(gpiob, gpiob.pb10), Segments::default()),
                (gpio_init_low!(gpiob, gpiob.pb1), Segments::default()),
                (gpio_init_high!(gpiob, gpiob.pb15), Segments::default()),
            ],
            gpioa,
        }
    }

    pub fn show(&mut self, timer: &mut Timer) {
        for (dig, seg) in self.segments_n.iter_mut() {
            seg.set_gpio(&self.gpioa);
            dig.set_high();
            timer.delay();
            dig.set_low();
        }
    }

    /// 设置特定位的数码管显示
    #[allow(dead_code)]
    pub fn set_display(&mut self, n: usize, dis: impl SegDisplay) {
        self.segments_n[n].1.state = dis.bits();
    }

    pub fn set_tube(&mut self, tube: impl TubeDisplay) {
        for (i, seg) in tube.tube_bits().into_iter().enumerate() {
            self.segments_n[i].1.state = seg.bits();
        }
    }
}
