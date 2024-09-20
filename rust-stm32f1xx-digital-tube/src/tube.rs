use stm32f1xx_hal::{
    gpio::{ErasedPin, GpioExt, IOPinSpeed, Output, OutputSpeed, PinState},
    pac::{GPIOA, GPIOB},
};

macro_rules! init_gpio {
    ($gpio:expr, $mask:expr) => {
        $gpio.crl.modify(|r, w| {
            let mut crl = r.bits();
            for i in 0..=7 {
                let inuse = ($mask >> i) & 1;
                if inuse == 1 {
                    crl &= !(0b1111 << (i * 4));
                    // 0001: 通用推免输出，最大速度10MHz
                    // 0010: 通用推免输出，最大速度2MHz
                    crl |= 0b0001 << (i * 4);
                }
            }
            unsafe { w.bits(crl) }
        });
        $gpio.crh.modify(|r, w| {
            let mut crh = r.bits();
            for i in 8..=15 {
                let inuse = ($mask >> i) & 1;
                if inuse == 1 {
                    crh &= !(0b1111 << (i * 4));
                    // 0001: 通用推免输出，最大速度10MHz
                    // 0010: 通用推免输出，最大速度2MHz
                    crh |= 0b0001 << (i * 4);
                }
            }
            unsafe { w.bits(crh) }
        })
    };
}

pub const SEGMENTS_MASK: u32 = 0b0001_1111_1000_0000;

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
pub struct Segments {
    state: u32,
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
        init_gpio!(gpioa, SEGMENTS_MASK);
    }

    /// 改变当前控制的数码管显示状态，需要先改变当前控制的数码管
    pub fn update(&self, gpioa: &GPIOA) {
        gpioa.odr.modify(|r, w| unsafe {
            w.bits((r.bits() & !SEGMENTS_MASK) | (self.state & SEGMENTS_MASK))
        });
    }

    pub fn set_display<D: SegDisplay>(&mut self) {
        self.state = D::STATE;
    }
}

pub trait SegDisplay {
    const STATE: u32;
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
        let mut io = $pin.into_push_pull_output_with_state(&mut $gpio.crl, PinState::High);
        io.set_speed(&mut $gpio.crl, IOPinSpeed::Mhz2);
        io.erase()
    }};
}

macro_rules! gpio_init_high {
    ($gpio:expr, $pin: expr) => {{
        let mut io = $pin.into_push_pull_output_with_state(&mut $gpio.crh, PinState::High);
        io.set_speed(&mut $gpio.crh, IOPinSpeed::Mhz2);
        io.erase()
    }};
}

impl Tube {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB) -> Tube {
        static mut INITED: bool = false;
        assert!(!unsafe { INITED }, "Tube has been initialized");
        Segments::init(&gpioa);
        let mut gpiob = gpiob.split();

        unsafe {
            INITED = true;
        };
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

    pub fn update(&mut self) {
        for (dig, seg) in self.segments_n.iter_mut() {
            dig.set_low();
            seg.update(&self.gpioa);
            dig.set_low();
        }
    }

    /// 设置特定位的数码管显示
    pub fn set_display<D: SegDisplay>(&mut self, n: usize) {
        self.segments_n[n].1.set_display::<D>();
    }
}
