use crate::*;

pub(crate) fn sort(x: &mut List) {
    for bit in 0..=(x.len().ilog2() as usize) {
        sort_by_bit(x, bit);
    }
}

pub fn sort_by_bit(x: &mut List, bit: usize) {
    let mut v = vec![];

    for i in 0..(x.len()) {
        let r = x.get(i);
        if !n_th_bit(r, bit) {
            v.push(r)
        }
    }

    for i in 0..(x.len()) {
        let r = x.get(i);
        if n_th_bit(r, bit) {
            v.push(r)
        }
    }

    for (i, value) in v.into_iter().enumerate() {
        x.set(i, value);
    }
    // *x = a;
}

fn n_th_bit(x: usize, bit: usize) -> bool {
    (x >> bit) & 1 == 1
}
