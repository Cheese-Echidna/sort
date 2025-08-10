use crate::sketch::*;

pub fn sort(ray: &mut List) {
    for i in 0..(ray.len() - 1) {
        let min_func = |i: usize, ray: &mut List| (i, ray.get(i));
        let mut min = min_func(i, ray);
        for j in (i + 1)..ray.len() {
            if ray.get(j) < min.1 {
                min = min_func(j, ray);
            }
        }
        ray.swap(i, min.0);
    }
}
