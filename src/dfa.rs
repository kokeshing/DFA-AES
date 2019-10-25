use crate::aes::*;
use crate::galoafield::GF2_8;

pub fn dfa(out: &State, broken: &State, y_list: Vec<[GF2_8; 4]>) -> Vec<[GF2_8; 4]> {
    let [diff0, diff1, diff2, diff3] = extract_diff(out, broken);

    let new_y_list: Vec<[GF2_8; 4]> = Vec::new();
    for z in (0..256).map(|v| GF2_8(v as u8)) {
        for [y0, y1, y2, y3] in y_list {
            for [c0, c1, c2, c3] in C_LIST {
                if diff0 == s(y0) + s(GF2_8(c0) * z + y0)
                    && diff1 == s(y1) + s(GF2_8(c1) * z + y1)
                    && diff2 == s(y2) + s(GF2_8(c2) * z + y2)
                    && diff3 == s(y3) + s(GF2_8(c3) * z + y3)
                {
                    new_y_list.append([y0, y1, y2, y3]);
                }
            }
        }
    }

    return new_y_list;
}

fn extract_diff(out: &State, broken: &State) -> [GF2_8; 4] {
    add_state(out, broken).iter().flatten().collect()
}

fn diff_index(out: &State, broken: &State) -> Vec<usize> {
    add_state(out, broken)
        .iter()
        .flatten()
        .zip((0..16))
        .filter(|v, _| v != GF(0))
        .unzip()
        .1
}

const C_LIST: [[u8; 4]; 4] = [
    [1, 1, 2, 3],
    [1, 1, 3, 2],
    [1, 2, 1, 3],
    [1, 2, 3, 1],
    [1, 3, 1, 2],
    [1, 3, 2, 1],
    [1, 1, 2, 3],
    [1, 1, 3, 2],
    [1, 2, 1, 3],
    [1, 2, 3, 1],
    [1, 3, 1, 2],
    [1, 3, 2, 1],
    [2, 1, 1, 3],
    [2, 1, 3, 1],
    [2, 1, 1, 3],
    [2, 1, 3, 1],
    [2, 3, 1, 1],
    [2, 3, 1, 1],
    [3, 1, 1, 2],
    [3, 1, 2, 1],
    [3, 1, 1, 2],
    [3, 1, 2, 1],
    [3, 2, 1, 1],
    [3, 2, 1, 1],
];
