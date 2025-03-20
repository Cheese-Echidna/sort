use crate::*;

pub(crate) fn sort(x: &mut impl ListPart) {
    if x.len() <= 1 {
        return;
    }
    let mid = x.len() / 2;

    {
        let mut left = x.slice(0..mid);
        sort(&mut left);
    }
    {
        let mut right = x.slice(mid..x.len());
        sort(&mut right);
    }

    merge(x, mid);
}

fn merge(x: &mut impl ListPart, mid: usize) {
    let len = x.len();
    let mut merged = Vec::with_capacity(len);
    let (mut i, mut j) = (0, mid);

    while i < mid && j < len {
        if x.get(i) <= x.get(j) {
            merged.push(x.get(i));
            i += 1;
        } else {
            merged.push(x.get(j));
            j += 1;
        }
    }

    while i < mid {
        merged.push(x.get(i));
        i += 1;
    }

    while j < len {
        merged.push(x.get(j));
        j += 1;
    }

    for (k, value) in merged.into_iter().enumerate() {
        x.set(k, value);
    }
}
