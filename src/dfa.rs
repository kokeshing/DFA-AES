use crate::aes::*;
use crate::galoafield::GF2_8;

pub fn dfa(out: &State, broken: &State, y_list: Vec<[GF2_8; 4]>) -> Vec<[GF2_8; 4]> {
    let mut new_y_list: Vec<[GF2_8; 4]> = Vec::new();

    let diff = extract_diff(out, broken);
    if diff.len() != 4 {
        println!("broken cyphertext is not valid");
        return new_y_list;
    }

    let diff_index = extract_diff_index(out, broken);
    let [c0, c1, c2, c3] = C_LIST[diff_index[0]];

    for z in (0..256).map(|v| GF2_8(v as u8)) {
        for [y0, y1, y2, y3] in y_list.iter() {
            if diff[0] == s(*y0) + s(GF2_8(c0) * z + *y0)
                && diff[1] == s(*y1) + s(GF2_8(c1) * z + *y1)
                && diff[2] == s(*y2) + s(GF2_8(c2) * z + *y2)
                && diff[3] == s(*y3) + s(GF2_8(c3) * z + *y3)
            {
                new_y_list.push([*y0, *y1, *y2, *y3]);
            }
        }
    }

    return new_y_list;
}

fn extract_diff(out: &State, broken: &State) -> Vec<GF2_8> {
    add_state(out, broken)
        .iter()
        .flatten()
        .filter(|&v| v != &GF2_8(0))
        .cloned()
        .collect()
}

pub fn extract_diff_index(out: &State, broken: &State) -> Vec<usize> {
    let (_, index): (Vec<GF2_8>, Vec<usize>) = add_state(out, broken)
        .iter()
        .flatten()
        .zip(0..16)
        .filter(|(&v, _i)| v != GF2_8(0))
        .unzip();

    return index;
}

pub fn all_y_list() -> Vec<[GF2_8; 4]> {
    let mut all_y_list: Vec<[GF2_8; 4]> = Vec::new();
    for y0 in (0..256).map(|v| GF2_8(v as u8)) {
        for y1 in (0..256).map(|v| GF2_8(v as u8)) {
            for y2 in (0..256).map(|v| GF2_8(v as u8)) {
                for y3 in (0..256).map(|v| GF2_8(v as u8)) {
                    all_y_list.push([y0, y1, y2, y3])
                }
            }
        }
    }

    return all_y_list;
}

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

const C_LIST: [[u8; 4]; 4] = [[2, 1, 1, 3], [3, 2, 1, 1], [1, 3, 2, 1], [1, 1, 3, 2]];
