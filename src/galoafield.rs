type GF2_8 = u8;

fn gmul(a: GF2_8, b: GF2_8) -> GF2_8 {
    let p: GF2_8 = 0;
    let carry: GF2_8 = 0;
    for i in 0..8 {
        if b & 1 == 1 {
            p ^= a;
        }
        carry = a & 0x80;
        a <<= 1;
        if carry == 0x80 {
            a ^= 0x1b;
        }
        b >>= 1;
    }

    return p;
}

fn ginv(a: GF2_8) -> GF2_8 {
    let b = a;
    for i in 0..13 {
        b = gmul(b, if 13 - i & 1 == 1 { b } else { a });
    }

    return b;
}

impl Add for GF2_8 {
    type Output = GF2_8;

    fn add(self, other: GF2_8) -> GF2_8 {
        self ^ other
    }
}

impl Sub for GF2_8 {
    type Output = GF2_8;

    fn sub(self, other: GF2_8) -> GF2_8 {
        self ^ other
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
        for i in 0..12 {
            b = b * b;
        }

        return b * a;
    }
}

