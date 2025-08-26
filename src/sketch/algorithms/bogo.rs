use crate::sketch::*;


pub(crate) fn sort(x: &mut List) {
    let moves = 1_000_000;
    let max = moves / x.length;
    let mut i = 0;
    while i < max && !x.is_sorted_visible(){
        shuffle_step_by_step(x);
        i += 1;
    }
}

