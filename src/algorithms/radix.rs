use crate::*;

pub(crate) fn sort(x: &mut List) {
    for bit in 0..(x.len().ilog2() as usize) {
        sort_by_bit(x, bit);
    }
}

pub fn sort_by_bit(x: &mut List, bit: usize) {
    let mut v1 = vec![];
    let mut v2 = vec![];

    for i in 0..x.len() {
        let r = x.get(i);
        if !n_th_bit(r, bit) {
            v1.push(r)
        } else {
            v2.push(r)
        }
    }
    let len = v1.len();
    for (i, value) in v1.into_iter().enumerate() {
        x.set(i, value);
    }
    for (i, value) in v2.into_iter().enumerate() {
        x.set(i + len, value);
    }
    // *x = a;
}

fn n_th_bit(x: usize, bit: usize) -> bool {
    (x >> bit) & 1 == 1
}
