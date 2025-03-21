use crate::*;

pub(crate) fn sort(x: &mut List) {
    let mut buckets = vec![0; x.len()];
    // for bit in 0..=(x.len().ilog2() as usize) {
    //     sort_by_bit(x, bit);
    // }
    for i in 0..x.len() {
        let v = x.get(i);
        buckets[v] += 1;
    }

    let mut i = 0;
    for (v, q) in buckets.into_iter().enumerate() {
        for j in i..(q+i) {
            x.set(j, v)
        }
        i += q;
    }
}

