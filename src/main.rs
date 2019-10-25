mod aes;
mod dfa;
mod galoafield;

use crate::aes::*;
use crate::galoafield::GF2_8;

fn main() {
    let diff_0 = GF2_8(0xe7);
    let diff_1 = GF2_8(0x51);
    let diff_2 = GF2_8(0x47);
    let diff_3 = GF2_8(0x99);
    for z in (0..256).map(|v| GF2_8(v as u8)) {
        for y_0 in
            (0..256).filter(|&v| diff_0 == s(GF2_8(v as u8)) + s(GF2_8(2) * z + GF2_8(v as u8)))
        {
            for y_1 in
                (0..256).filter(|&v| diff_1 == s(GF2_8(v as u8)) + s(GF2_8(3) * z + GF2_8(v as u8)))
            {
                for y_2 in (0..256).filter(|&v| diff_2 == s(GF2_8(v as u8)) + s(z + GF2_8(v as u8)))
                {
                    for y_3 in
                        (0..256).filter(|&v| diff_3 == s(GF2_8(v as u8)) + s(z + GF2_8(v as u8)))
                    {
                        println!(
                            "z: {:02x}, y_0: {:02x}, y_1: {:02x}, y_2: {:02x}, y_3: {:02x}",
                            z.0, y_0, y_1, y_2, y_3
                        );
                    }
                }
            }
        }
    }
}
