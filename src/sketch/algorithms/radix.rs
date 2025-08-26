use crate::sketch::*;

pub(crate) fn sort(x: &mut List, base: usize) {
    for n in 0..((x.len() as f64).log(base as f64).ceil() as usize) {
        sort_by_base_n(x, base, n);
    }
}

pub  fn sort_by_base_n(x: &mut List, base: usize, n: usize) {
    let mut buckets: Vec<Vec<usize>> = vec![vec![]; base];
    let bp = base.pow(n as u32);
    for i in 0..x.len() {
        let v = x.get(i);
        let rem = (v / bp) % base;
        buckets[rem].push(v)
    }
    buckets.into_iter().flatten().enumerate().for_each(|(i, v)| x.set(i, v));
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