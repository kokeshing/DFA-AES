mod aes;
mod galoafield;

use crate::aes::*;
use crate::galoafield::GF2_8;

fn main() {
    let mut s: State = [[GF2_8(0); 4]; 4];

    let key: State = [
        [GF2_8(0), GF2_8(4), GF2_8(8), GF2_8(12)],
        [GF2_8(1), GF2_8(5), GF2_8(9), GF2_8(13)],
        [GF2_8(2), GF2_8(6), GF2_8(0), GF2_8(14)],
        [GF2_8(3), GF2_8(7), GF2_8(1), GF2_8(15)],
    ];

    print_state(&s);
    print_state(&key);

    let w = key_expansion(key);

    let mut round_key = set_key(w, 0);
    s = add_state(s, round_key);

    for i in 0..10 {
        s = sub_byte(s);
        s = shift_rows(s);

        if i != 9 {
            s = mix_column(s);
        }

        round_key = set_key(w, 0);
        s = add_state(s, round_key);
    }

    print_state(&s);
}
