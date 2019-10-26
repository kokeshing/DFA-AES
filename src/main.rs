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
    let ciphertext = str_to_state(s).expect("Failed to parse the ciphertext.");

    let mut broken_list: Vec<State> = Vec::new();
    for i in 0..8 {
        println!("Input the {} broken ciphertext.", i);
        stdin()
            .read_line(&mut s)
            .expect("Failed to read the broken ciphertext.");
        let brokentext = str_to_state(s).expect("Failed to parse the broken ciphertext.");
        broken_list.push(brokentext);
    }
    broken_list.sort_by(|a, b| {
        diff_index(&ciphertext, &a)[0]
            .partial_cmp(&diff_index(&ciphertext, &b)[0])
            .unwrap()
    });

    let mut y: State = [[GF2_8(0); 4]; 4];
    for i in (0..broken_list.len()).step_by(2) {
        let mut y_list = new_y_list();
        y_list = dfa(&ciphertext, &broken_list[i], y_list);
        y_list = dfa(&ciphertext, &broken_list[i + 1], y_list);
        if y_list.len() == 1 {
            for (index, j) in diff_index(&ciphertext, &broken_list[i]).iter().enumerate() {
                y[j / 4][j % 4] = y_list[0][index];
            }
        }
    }

    print_state_map(&y);
    print_state(&y);
}
