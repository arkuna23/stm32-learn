use crate::tube::{SegDisplay, Segment, TubeDisplay};

macro_rules! tube_display {
    ($name:ident, $bits:expr) => {
        pub struct $name;
        impl $name {
            const BITS: u32 = $bits;
        }
        impl SegDisplay for $name {
            #[inline(always)]
            fn bits(self) -> u32 {
                Self::BITS
            }
        }
    };
}

tube_display!(
    Num0,
    Segment::A | Segment::B | Segment::C | Segment::D | Segment::E | Segment::F
);
tube_display!(Num1, Segment::B | Segment::C);
tube_display!(
    Num2,
    Segment::A | Segment::B | Segment::G | Segment::E | Segment::D
);
tube_display!(
    Num3,
    Segment::A | Segment::B | Segment::G | Segment::C | Segment::D
);
tube_display!(Num4, Segment::F | Segment::G | Segment::B | Segment::C);
tube_display!(
    Num5,
    Segment::A | Segment::F | Segment::G | Segment::C | Segment::D
);
tube_display!(
    Num6,
    Segment::A | Segment::F | Segment::G | Segment::C | Segment::D | Segment::E
);
tube_display!(Num7, Segment::A | Segment::B | Segment::C);
tube_display!(
    Num8,
    Segment::A | Segment::B | Segment::C | Segment::D | Segment::E | Segment::F | Segment::G
);
tube_display!(
    Num9,
    Segment::A | Segment::B | Segment::C | Segment::D | Segment::F | Segment::G
);

#[inline(always)]
pub fn digit_to_segments(digit: u8) -> u32 {
    match digit {
        0 => Num0.bits(),
        1 => Num1.bits(),
        2 => Num2.bits(),
        3 => Num3.bits(),
        4 => Num4.bits(),
        5 => Num5.bits(),
        6 => Num6.bits(),
        7 => Num7.bits(),
        8 => Num8.bits(),
        9 => Num9.bits(),
        _ => panic!("Invalid digit"),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Number(pub u16);

impl TubeDisplay for Number {
    fn tube_bits(self) -> [impl SegDisplay; 4] {
        assert!(self.0 < 10000, "Number out of range");
        let mut number = self.0;
        let mut segs = [0u32; 4];
        (0..4).for_each(|i| {
            segs[i] = {
                let digit: u8 = (number % 10) as u8;
                number /= 10;
                digit_to_segments(digit).combine(Segment::DP)
            }
        });
        segs
    }
}
