mod aes;
mod dfa;
mod galoafield;

use crate::aes::*;
use crate::dfa::*;
use crate::galoafield::GF2_8;
use std::io::stdin;

fn main() {
    let mut s = String::new();

    // 正常な暗号文を読み込む
    println!("Input the ciphertext.");
    stdin()
        .read_line(&mut s)
        .expect("Failed to read the ciphertext.");
    let ciphertext = str_to_state(s.trim().to_string()).expect("Failed to parse the ciphertext.");

    // 8つの壊れた暗号文を読み込む
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
    // 壊れた暗号文の故障箇所でソートして故障が同じ列のペアでまとめる
    broken_list.sort_by(|a, b| {
        extract_diff_index(&ciphertext, &a)[0]
            .partial_cmp(&extract_diff_index(&ciphertext, &b)[0])
            .unwrap()
    });

    // 故障した暗号文を2つでDFAを行って4バイト特定するのを4回繰り返す
    let mut y: State = [[GF2_8(0); 4]; 4];
    for i in (0..broken_list.len()).step_by(2) {
        let a = dfa(&ciphertext, &broken_list[i]);
        let b = dfa(&ciphertext, &broken_list[i + 1]);
        let y_list: Vec<_> = a.intersection(&b).collect(); // 2つの故障した暗号文から求めた候補から共通のものを取り出す
        if y_list.len() == 1 {
            for (index, j) in extract_diff_index(&ciphertext, &broken_list[i])
                .iter()
                .enumerate()
            {
                y[j / 4][j % 4] = y_list[0][index]; // 結果を格納
            }
        } else {
            println!("y_list length is not 1 at {}", i);
        }
    }

    // 求めた10ラウンド目の開始時のStateの値を表示
    println!("y:");
    print_state_map(&y);

    // 求めた10ラウンド目の開始時のStateをSubByteしてから正常な暗号文を足すことで10ラウンド目の鍵を求める
    y = sub_byte(y);
    let key = add_state(&ciphertext, &y);

    println!("\nThe key of 10 round");
    print_state(&key);
    print_state_map(&key);
}
