use crate::aes::*;
use crate::galoafield::GF2_8;
use std::collections::HashSet;

// 正常な暗号文と故障した暗号文からあり得る10ラウンド目の開始時の故障箇所のStateの値の組を返す
pub fn dfa(out: &State, broken: &State) -> HashSet<[GF2_8; 4]> {
    let mut y_list: HashSet<[GF2_8; 4]> = HashSet::new();

    // 故障箇所の差分を取る
    let diff = extract_diff(out, broken);
    if diff.len() != 4 {
        println!("broken cyphertext is not valid");
        return y_list;
    }

    // 注入されたの故障による差分zと係数の組cを仮定し故障箇所全てにおいて 最終的な差分 と s(y) + s(c * z + y) が一致するyの組を返す
    for z in (0..256).map(|v| GF2_8(v as u8)) {
        for [c0, c1, c2, c3] in C_LIST.iter() {
            for y0 in (0..256)
                .map(|v| GF2_8(v as u8))
                .filter(|&v| diff[0] == s(v) + s(*c0 * z + v))
            {
                for y1 in (0..256)
                    .map(|v| GF2_8(v as u8))
                    .filter(|&v| diff[1] == s(v) + s(*c1 * z + v))
                {
                    for y2 in (0..256)
                        .map(|v| GF2_8(v as u8))
                        .filter(|&v| diff[2] == s(v) + s(*c2 * z + v))
                    {
                        for y3 in (0..256)
                            .map(|v| GF2_8(v as u8))
                            .filter(|&v| diff[3] == s(v) + s(*c3 * z + v))
                        {
                            y_list.insert([y0, y1, y2, y3]);
                        }
                    }
                }
            }
        }
    }

    return y_list;
}

// 2つのStateの差分を取る
fn extract_diff(out: &State, broken: &State) -> Vec<GF2_8> {
    add_state(out, broken)
        .iter()
        .flatten()
        .filter(|&v| v != &GF2_8(0))
        .cloned()
        .collect()
}

// 2つのStateの差分の位置を返す
pub fn extract_diff_index(out: &State, broken: &State) -> Vec<usize> {
    let (_, index): (Vec<GF2_8>, Vec<usize>) = add_state(out, broken)
        .iter()
        .flatten()
        .zip(0..16)
        .filter(|(&v, _i)| v != GF2_8(0))
        .unzip();

    return index;
}

// 上の2つの関数が想定通り動くかのテスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_diff_test() {
        let a: State = [
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
        ];
        let b: State = [
            [GF2_8(1), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(2)],
            [GF2_8(0), GF2_8(0), GF2_8(3), GF2_8(0)],
            [GF2_8(0), GF2_8(4), GF2_8(0), GF2_8(0)],
        ];
        assert_eq!(
            extract_diff(&a, &b),
            vec![GF2_8(1), GF2_8(2), GF2_8(3), GF2_8(4)]
        );
    }

    #[test]
    fn diff_index_test() {
        let a: State = [
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(0)],
        ];
        let b: State = [
            [GF2_8(1), GF2_8(0), GF2_8(0), GF2_8(0)],
            [GF2_8(0), GF2_8(0), GF2_8(0), GF2_8(2)],
            [GF2_8(0), GF2_8(0), GF2_8(3), GF2_8(0)],
            [GF2_8(0), GF2_8(4), GF2_8(0), GF2_8(0)],
        ];
        assert_eq!(extract_diff_index(&a, &b), vec![0, 7, 10, 13]);
    }
}

const C_LIST: [[GF2_8; 4]; 4] = [
    [GF2_8(2), GF2_8(1), GF2_8(1), GF2_8(3)],
    [GF2_8(3), GF2_8(2), GF2_8(1), GF2_8(1)],
    [GF2_8(1), GF2_8(3), GF2_8(2), GF2_8(1)],
    [GF2_8(1), GF2_8(1), GF2_8(3), GF2_8(2)],
];
