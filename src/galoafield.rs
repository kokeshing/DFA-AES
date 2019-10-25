use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct GF2_8(pub u8);

pub fn gmul(a: GF2_8, b: GF2_8) -> GF2_8 {
    let mut p: u8 = 0;
    let mut carry: u8;
    let mut _a: u8 = a.0;
    let mut _b: u8 = b.0;
    for _ in 0..8 {
        if _b & 1 == 1 {
            p ^= _a;
        }
        carry = _a & 0x80;
        _a <<= 1;
        if carry == 0x80 {
            _a ^= 0x1b;
        }
        _b >>= 1;
    }

    return GF2_8(p);
}

pub fn ginv(a: GF2_8) -> GF2_8 {
    let mut b = a;
    for i in 0..13 {
        b = gmul(b, if 13 - i & 1 == 1 { b } else { a });
    }

    return b;
}

impl Add for GF2_8 {
    type Output = GF2_8;

    fn add(self, other: GF2_8) -> GF2_8 {
        GF2_8(self.0 ^ other.0)
    }
}

impl Sub for GF2_8 {
    type Output = GF2_8;

    fn sub(self, other: GF2_8) -> GF2_8 {
        GF2_8(self.0 ^ other.0)
    }
}

impl Mul for GF2_8 {
    type Output = GF2_8;

    fn mul(self, other: GF2_8) -> GF2_8 {
        gmul(self, other)
    }
}

impl Div for GF2_8 {
    type Output = GF2_8;

    fn div(self, other: GF2_8) -> GF2_8 {
        let inv_other = ginv(other);

        return gmul(self, inv_other);
    }
}

impl PartialEq for GF2_8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for GF2_8 {}
