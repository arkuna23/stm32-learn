use stm32f1xx_hal::{pac::Peripherals, stm32};

macro_rules! init_gpio {
    ($gpio:expr, $mask:expr) => {
        $gpio.crl.modify(|r, w| {
            let mut crl = r.bits();
            for i in 0..=7 {
                let inuse = ($mask >> i) & 1;
                if inuse == 1 {
                    crl &= !(0b1111 << (i * 4));
                    crl |= 0b0010 << (i * 4);
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
                    crh |= 0b0010 << (i * 4);
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
    pub fn init(dp: &stm32::Peripherals) {
        init_gpio!(dp.GPIOA, SEGMENTS_MASK);
    }

    pub fn update(&self, dp: &stm32::Peripherals) {
        dp.GPIOA.odr.modify(|r, w| unsafe {
            w.bits((r.bits() & !SEGMENTS_MASK) | (self.state & SEGMENTS_MASK))
        });
    }

    pub fn set_display<D: SegDisplay>(&mut self) {
        self.state = D::state();
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
pub trait SegDisplay {
    fn state() -> u32;
}

pub const TUBE_MASK: u32 = 0b1000_1100_0000_0010;

/// 5461BS-1 4位
/// [文档](http://www.xlitx.com/datasheet/5461BS.pdf)
/// DIG1(12) -> PB11
/// DIG2(9) -> PB10
/// DIG3(8) -> PB1
/// DIG4(6) -> PB15
pub struct Tube<'a> {
    segments_n: [Segments; 4],
    dp: &'a stm32::Peripherals,
}
impl<'a> Tube<'a> {
    pub fn init(dp: &'a Peripherals) {
        static mut INITED: bool = false;
        assert!(!unsafe { INITED }, "Tube has been initialized");
        Segments::init(dp);
        init_gpio!(dp.GPIOB, TUBE_MASK);
        unsafe {
            INITED = true;
        }
    }

    pub fn new(dp: &'a Peripherals) -> Tube<'a> {
        Self {
            segments_n: Default::default(),
            dp,
        }
    }

    pub fn update(&self) {
        for seg in self.segments_n.iter() {
            seg.update(self.dp);
        }
    }

    pub fn set_display<D: SegDisplay>(&mut self, n: usize) {
        self.segments_n[n].set_display::<D>();
    }
}
