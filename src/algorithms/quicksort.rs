use crate::*;

pub fn sort(ray: &mut impl ListPart) {
    // Base case: if the list is empty or has one element, it's already sorted.
    if ray.len() <= 1 {
        return;
    }

    // Use the first element as the pivot.
    let pivot = ray.get(0);
    let len = ray.len();

    // i marks the start of the region for elements greater than or equal to pivot.
    let mut i = 1;

    // Partition the list into two parts: < pivot and >= pivot.
    for j in 1..len {
        if ray.get(j) < pivot {
            ray.swap(i, j);
            i += 1;
        }
    }

    // Place the pivot into its correct position.
    ray.swap(0, i - 1);

    // Recursively sort the left and right partitions.
    let mut left = ray.slice(0..(i - 1));
    sort(&mut left);
    let mut right = ray.slice(i..len);
    sort(&mut right);
}
