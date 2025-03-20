use crate::*;

pub fn sort(ray: &mut List) {
    for i in 0..(ray.len() - 1) {
        let mut min = i;
        for j in min..(ray.len()) {
            if ray.get(j) < ray.get(min) {
                min = j;
            }
        }
        ray.swap(i, min);
    }
}