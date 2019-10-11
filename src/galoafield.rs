use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct GF2_8(pub u8);

pub fn gmul(a: GF2_8, b: GF2_8) -> GF2_8 {
    let mut p = GF2_8(0);
    let mut carry = GF2_8(0);
    let mut _a = a;
    let mut _b = b;
    for _ in 0..8 {
        if _b.0 & 1 == 1 {
            p.0 ^= _a.0;
        }
        carry.0 = _a.0 & 0x80;
        _a.0 <<= 1;
        if carry.0 == 0x80 {
            _a.0 ^= 0x1b;
        }
        _b.0 >>= 1;
    }

    return p;
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
