mod aes;
mod dfa;
mod galoafield;

use crate::aes::*;
use crate::galoafield::GF2_8;

fn main() {
    /*
    let mut s: State =
        str_to_state("00000000000000000000000000000000".to_string()).expect("Failed parse input");
    let key: State = str_to_state("000102030405060708090a0b0c0d0e0f".to_string()).expect("Failed parse key");
    let c = aes_encrypt(&s, &key);
    print_state(&c);
    */

    let mut s: State = str_to_state("456c6b2eb68108431e20e253174f739d".to_string()).expect("");
    let mut s_broken: State =
        str_to_state("45266b2e168108431e20e2f4174f379d".to_string()).expect("");

    s = inv_sub_byte(s);
    s_broken = inv_sub_byte(s_broken);

    let mut diff = add_state(s, s_broken);
    print_state_map(&diff);
}
