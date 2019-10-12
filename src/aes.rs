use crate::galoafield::GF2_8;
use std::io::{stdout, Write};

pub type State = [[GF2_8; 4]; 4];

pub fn print_state_map(s: &State) -> () {
    let out = stdout();
    let mut out = out.lock();
    for i in 0..4 {
        write!(out, "{}", if i == 0 { "[" } else { " " }).unwrap();
        write!(
            out,
            "[{:02X}, {:02X}, {:02X}, {:02X}]",
            s[i][0].0, s[i][1].0, s[i][2].0, s[i][3].0
        )
        .unwrap();
        write!(out, "{}", if i == 3 { "]\n" } else { "\n" }).unwrap();
    }
}

pub fn print_state(s: &State) -> () {
    let out = stdout();
    let mut out = out.lock();
    write!(out, "0x").unwrap();
    for i in 0..4 {
        write!(
            out,
            "{:02X}{:02X}{:02X}{:02X}",
            s[0][i].0, s[1][i].0, s[2][i].0, s[3][i].0
        )
        .unwrap();
    }
    write!(out, "\n").unwrap();
}

pub fn add_state(a: State, b: State) -> State {
    let mut ret: State = [[GF2_8(0); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ret[i][j] = a[i][j] + b[i][j];
        }
    }

    return ret;
}

pub fn shift_rows(s: State) -> State {
    let mut ret: State = [[GF2_8(0); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ret[i][j] = s[i][(i + j) % 4];
        }
    }

    return ret;
}

const A: State = [
    [GF2_8(2), GF2_8(3), GF2_8(1), GF2_8(1)],
    [GF2_8(1), GF2_8(2), GF2_8(3), GF2_8(1)],
    [GF2_8(1), GF2_8(1), GF2_8(2), GF2_8(3)],
    [GF2_8(3), GF2_8(1), GF2_8(1), GF2_8(2)],
];
pub fn mix_column(s: State) -> State {
    let mut ret: State = [[GF2_8(0); 4]; 4];
    for i in 0..4 {
        for k in 0..4 {
            for j in 0..4 {
                ret[i][j] = ret[i][j] + A[i][k] * s[k][j];
            }
        }
    }

    return ret;
}

pub fn sub_byte(s: State) -> State {
    let mut ret: State = [[GF2_8(0); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ret[i][j] = SBOX[s[i][j].0 as usize];
        }
    }

    return ret;
}

pub fn key_expansion(k: State) -> [GF2_8; 176] {
    let mut w: [GF2_8; 176] = [GF2_8(0); 176];

    for i in 0..4 {
        for j in 0..4 {
            w[i * 4 + j] = k[j][i];
        }
    }

    let (mut t0, mut t1, mut t2, mut t3);
    for i in (16..176).step_by(4) {
        if i % 16 == 0 {
            t0 = w[i - 3];
            t1 = w[i - 2];
            t2 = w[i - 1];
            t3 = w[i - 4];

            t0 = SBOX[t0.0 as usize];
            t1 = SBOX[t1.0 as usize];
            t2 = SBOX[t2.0 as usize];
            t3 = SBOX[t3.0 as usize];

            t0 = t0 + RCON[i / 16];
            t1 = t1 + GF2_8(0);
            t2 = t2 + GF2_8(0);
            t3 = t3 + GF2_8(0);
        } else {
            t0 = w[i - 4];
            t1 = w[i - 3];
            t2 = w[i - 2];
            t3 = w[i - 1];
        }

        w[i] = w[i - 16] + t0;
        w[i + 1] = w[i - 15] + t1;
        w[i + 2] = w[i - 14] + t2;
        w[i + 3] = w[i - 13] + t3;
    }

    return w;
}

pub fn set_key(w: [GF2_8; 176], round_num: usize) -> State {
    let mut ret: State = [[GF2_8(0); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ret[j][i] = w[round_num * 16 + 4 * i + j];
        }
    }

    return ret;
}

const RCON: [GF2_8; 16] = [
    GF2_8(0x01),
    GF2_8(0x01),
    GF2_8(0x02),
    GF2_8(0x04),
    GF2_8(0x08),
    GF2_8(0x10),
    GF2_8(0x20),
    GF2_8(0x40),
    GF2_8(0x80),
    GF2_8(0x1b),
    GF2_8(0x36),
    GF2_8(0x6c),
    GF2_8(0xd8),
    GF2_8(0xab),
    GF2_8(0x4d),
    GF2_8(0x9a),
];

const SBOX: [GF2_8; 256] = [
    GF2_8(0x63),
    GF2_8(0x7c),
    GF2_8(0x77),
    GF2_8(0x7b),
    GF2_8(0xf2),
    GF2_8(0x6b),
    GF2_8(0x6f),
    GF2_8(0xc5),
    GF2_8(0x30),
    GF2_8(0x01),
    GF2_8(0x67),
    GF2_8(0x2b),
    GF2_8(0xfe),
    GF2_8(0xd7),
    GF2_8(0xab),
    GF2_8(0x76),
    GF2_8(0xca),
    GF2_8(0x82),
    GF2_8(0xc9),
    GF2_8(0x7d),
    GF2_8(0xfa),
    GF2_8(0x59),
    GF2_8(0x47),
    GF2_8(0xf0),
    GF2_8(0xad),
    GF2_8(0xd4),
    GF2_8(0xa2),
    GF2_8(0xaf),
    GF2_8(0x9c),
    GF2_8(0xa4),
    GF2_8(0x72),
    GF2_8(0xc0),
    GF2_8(0xb7),
    GF2_8(0xfd),
    GF2_8(0x93),
    GF2_8(0x26),
    GF2_8(0x36),
    GF2_8(0x3f),
    GF2_8(0xf7),
    GF2_8(0xcc),
    GF2_8(0x34),
    GF2_8(0xa5),
    GF2_8(0xe5),
    GF2_8(0xf1),
    GF2_8(0x71),
    GF2_8(0xd8),
    GF2_8(0x31),
    GF2_8(0x15),
    GF2_8(0x04),
    GF2_8(0xc7),
    GF2_8(0x23),
    GF2_8(0xc3),
    GF2_8(0x18),
    GF2_8(0x96),
    GF2_8(0x05),
    GF2_8(0x9a),
    GF2_8(0x07),
    GF2_8(0x12),
    GF2_8(0x80),
    GF2_8(0xe2),
    GF2_8(0xeb),
    GF2_8(0x27),
    GF2_8(0xb2),
    GF2_8(0x75),
    GF2_8(0x09),
    GF2_8(0x83),
    GF2_8(0x2c),
    GF2_8(0x1a),
    GF2_8(0x1b),
    GF2_8(0x6e),
    GF2_8(0x5a),
    GF2_8(0xa0),
    GF2_8(0x52),
    GF2_8(0x3b),
    GF2_8(0xd6),
    GF2_8(0xb3),
    GF2_8(0x29),
    GF2_8(0xe3),
    GF2_8(0x2f),
    GF2_8(0x84),
    GF2_8(0x53),
    GF2_8(0xd1),
    GF2_8(0x00),
    GF2_8(0xed),
    GF2_8(0x20),
    GF2_8(0xfc),
    GF2_8(0xb1),
    GF2_8(0x5b),
    GF2_8(0x6a),
    GF2_8(0xcb),
    GF2_8(0xbe),
    GF2_8(0x39),
    GF2_8(0x4a),
    GF2_8(0x4c),
    GF2_8(0x58),
    GF2_8(0xcf),
    GF2_8(0xd0),
    GF2_8(0xef),
    GF2_8(0xaa),
    GF2_8(0xfb),
    GF2_8(0x43),
    GF2_8(0x4d),
    GF2_8(0x33),
    GF2_8(0x85),
    GF2_8(0x45),
    GF2_8(0xf9),
    GF2_8(0x02),
    GF2_8(0x7f),
    GF2_8(0x50),
    GF2_8(0x3c),
    GF2_8(0x9f),
    GF2_8(0xa8),
    GF2_8(0x51),
    GF2_8(0xa3),
    GF2_8(0x40),
    GF2_8(0x8f),
    GF2_8(0x92),
    GF2_8(0x9d),
    GF2_8(0x38),
    GF2_8(0xf5),
    GF2_8(0xbc),
    GF2_8(0xb6),
    GF2_8(0xda),
    GF2_8(0x21),
    GF2_8(0x10),
    GF2_8(0xff),
    GF2_8(0xf3),
    GF2_8(0xd2),
    GF2_8(0xcd),
    GF2_8(0x0c),
    GF2_8(0x13),
    GF2_8(0xec),
    GF2_8(0x5f),
    GF2_8(0x97),
    GF2_8(0x44),
    GF2_8(0x17),
    GF2_8(0xc4),
    GF2_8(0xa7),
    GF2_8(0x7e),
    GF2_8(0x3d),
    GF2_8(0x64),
    GF2_8(0x5d),
    GF2_8(0x19),
    GF2_8(0x73),
    GF2_8(0x60),
    GF2_8(0x81),
    GF2_8(0x4f),
    GF2_8(0xdc),
    GF2_8(0x22),
    GF2_8(0x2a),
    GF2_8(0x90),
    GF2_8(0x88),
    GF2_8(0x46),
    GF2_8(0xee),
    GF2_8(0xb8),
    GF2_8(0x14),
    GF2_8(0xde),
    GF2_8(0x5e),
    GF2_8(0x0b),
    GF2_8(0xdb),
    GF2_8(0xe0),
    GF2_8(0x32),
    GF2_8(0x3a),
    GF2_8(0x0a),
    GF2_8(0x49),
    GF2_8(0x06),
    GF2_8(0x24),
    GF2_8(0x5c),
    GF2_8(0xc2),
    GF2_8(0xd3),
    GF2_8(0xac),
    GF2_8(0x62),
    GF2_8(0x91),
    GF2_8(0x95),
    GF2_8(0xe4),
    GF2_8(0x79),
    GF2_8(0xe7),
    GF2_8(0xc8),
    GF2_8(0x37),
    GF2_8(0x6d),
    GF2_8(0x8d),
    GF2_8(0xd5),
    GF2_8(0x4e),
    GF2_8(0xa9),
    GF2_8(0x6c),
    GF2_8(0x56),
    GF2_8(0xf4),
    GF2_8(0xea),
    GF2_8(0x65),
    GF2_8(0x7a),
    GF2_8(0xae),
    GF2_8(0x08),
    GF2_8(0xba),
    GF2_8(0x78),
    GF2_8(0x25),
    GF2_8(0x2e),
    GF2_8(0x1c),
    GF2_8(0xa6),
    GF2_8(0xb4),
    GF2_8(0xc6),
    GF2_8(0xe8),
    GF2_8(0xdd),
    GF2_8(0x74),
    GF2_8(0x1f),
    GF2_8(0x4b),
    GF2_8(0xbd),
    GF2_8(0x8b),
    GF2_8(0x8a),
    GF2_8(0x70),
    GF2_8(0x3e),
    GF2_8(0xb5),
    GF2_8(0x66),
    GF2_8(0x48),
    GF2_8(0x03),
    GF2_8(0xf6),
    GF2_8(0x0e),
    GF2_8(0x61),
    GF2_8(0x35),
    GF2_8(0x57),
    GF2_8(0xb9),
    GF2_8(0x86),
    GF2_8(0xc1),
    GF2_8(0x1d),
    GF2_8(0x9e),
    GF2_8(0xe1),
    GF2_8(0xf8),
    GF2_8(0x98),
    GF2_8(0x11),
    GF2_8(0x69),
    GF2_8(0xd9),
    GF2_8(0x8e),
    GF2_8(0x94),
    GF2_8(0x9b),
    GF2_8(0x1e),
    GF2_8(0x87),
    GF2_8(0xe9),
    GF2_8(0xce),
    GF2_8(0x55),
    GF2_8(0x28),
    GF2_8(0xdf),
    GF2_8(0x8c),
    GF2_8(0xa1),
    GF2_8(0x89),
    GF2_8(0x0d),
    GF2_8(0xbf),
    GF2_8(0xe6),
    GF2_8(0x42),
    GF2_8(0x68),
    GF2_8(0x41),
    GF2_8(0x99),
    GF2_8(0x2d),
    GF2_8(0x0f),
    GF2_8(0xb0),
    GF2_8(0x54),
    GF2_8(0xbb),
    GF2_8(0x16),
];
