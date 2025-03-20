use crate::*;

pub fn sort(ray: &mut List) {
    for end in (0..ray.len()).rev() {
        for i in 0..end {
            if ray.get(i) > ray.get(i + 1) {
                ray.swap(i, i + 1)
            }
        }
    }
}
