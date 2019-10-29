mod aes;
mod dfa;
mod galoafield;

use crate::aes::*;
use crate::dfa::*;
use crate::galoafield::GF2_8;
use std::io::stdin;

fn main() {
    let mut s = String::new();

    println!("Input the ciphertext.");
    stdin()
        .read_line(&mut s)
        .expect("Failed to read the ciphertext.");
    let ciphertext = str_to_state(s.trim().to_string()).expect("Failed to parse the ciphertext.");

    let mut broken_list: Vec<State> = Vec::new();
    for i in 0..8 {
        let mut s = String::new();
        println!("Input the broken ciphertexts.({})", i);
        stdin()
            .read_line(&mut s)
            .expect("Failed to read the broken ciphertext.");
        let brokentext =
            str_to_state(s.trim().to_string()).expect("Failed to parse the broken ciphertext.");
        broken_list.push(brokentext);
    }
    broken_list.sort_by(|a, b| {
        extract_diff_index(&ciphertext, &a)[0]
            .partial_cmp(&extract_diff_index(&ciphertext, &b)[0])
            .unwrap()
    });

    let mut y: State = [[GF2_8(0); 4]; 4];
    for i in (0..broken_list.len()).step_by(2) {
        let a = dfa(&ciphertext, &broken_list[i]);
        let b = dfa(&ciphertext, &broken_list[i + 1]);
        let y_list: Vec<_> = a.intersection(&b).collect();
        if y_list.len() == 1 {
            println!("{} key sucess.", i);
            for (index, j) in extract_diff_index(&ciphertext, &broken_list[i])
                .iter()
                .enumerate()
            {
                y[j / 4][j % 4] = y_list[0][index];
            }
        } else {
            println!("y_list length is not 1. {:?}", y_list);
        }
    }

    print_state_map(&y);
    print_state(&y);

    y = sub_byte(y);
    let key = add_state(&ciphertext, &y);
    print_state(&key);
    print_state_map(&key);
}
