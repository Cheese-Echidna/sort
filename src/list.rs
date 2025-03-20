use nannou::prelude::Vec2;
use nannou::rand::seq::SliceRandom;
use nannou::rand::thread_rng;
use nannou::Draw;
use std::ops::Range;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct List {
    internal_vec: Vec<usize>,
    record_of_operations: Vec<Operation>,
    length: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Operation {
    Get(usize),
    Set(usize, usize),
    Swap(usize, usize),
}

// impl Operation {
//     fn is_draw(&self) -> bool {
//         match self {
//             Operation::Get(_) => false,
//             Operation::Set(_, _) => true,
//             Operation::Swap(_, _) => true,
//         }
//     }
//     fn fmt(&self) -> String {
//         match self {
//             Operation::Get(i) => {
//                 format!("list[{i}]")
//             }
//             Operation::Set(i, x) => {
//                 format!("list[{i}] = {x}")
//             }
//             Operation::Swap(i, j) => {
//                 format!("swap({i},{j})")
//             }
//         }
//     }
// }

impl List {
    pub fn new(vec: Vec<usize>, length: usize) -> Self {
        Self {
            internal_vec: vec.clone(),
            record_of_operations: vec![],
            length,
        }
    }
    fn record(&mut self, operation: Operation) {
        self.record_of_operations.push(operation);
    }
    #[allow(dead_code)]
    pub(crate) fn iter(&self) -> Iter<'_, usize> {
        self.internal_vec.iter()
    }
}

impl ListPart for List {
    fn get(&mut self, i: usize) -> usize {
        self.record(Operation::Get(i));
        self.internal_vec[i]
    }
    fn set(&mut self, i: usize, x: usize) {
        self.record(Operation::Set(i, x));
        self.internal_vec[i] = x;
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.record(Operation::Swap(i, j));
        self.internal_vec.swap(i, j);
    }
    fn slice(&mut self, range: Range<usize>) -> SliceOfList {
        SliceOfList { range, list: self }
    }
    fn len(&self) -> usize {
        self.length
    }
}

fn rect_corner_wh(bottom_left: Vec2, wh: Vec2) -> (Vec2, Vec2) {
    let center = bottom_left + wh / 2.;
    // let width = (corner1 - corner2).abs();
    (center, wh)
}

pub struct SliceOfList<'a> {
    range: Range<usize>,
    list: &'a mut List,
}

impl SliceOfList<'_> {
    fn sliced(&mut self) -> &mut [usize] {
        &mut self.list.internal_vec[self.range.clone()]
    }
    fn record(&mut self, operation: Operation) {
        let slice_start = self.range.start;
        let operation = match operation {
            Operation::Get(i) => Operation::Get(i + slice_start),
            Operation::Set(i, x) => Operation::Set(i + slice_start, x),
            Operation::Swap(i, j) => Operation::Swap(i + slice_start, j + slice_start),
        };
        self.list.record(operation);
    }
}

pub trait ListPart {
    fn get(&mut self, i: usize) -> usize;
    fn set(&mut self, i: usize, x: usize);
    fn swap(&mut self, i: usize, j: usize);
    fn slice(&mut self, range: Range<usize>) -> SliceOfList;
    fn len(&self) -> usize;
}

impl ListPart for SliceOfList<'_> {
    fn get(&mut self, i: usize) -> usize {
        self.record(Operation::Get(i));
        self.sliced()[i]
    }
    fn set(&mut self, i: usize, x: usize) {
        self.record(Operation::Set(i, x));
        self.sliced()[i] = x;
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.record(Operation::Swap(i, j));
        self.sliced().swap(i, j);
    }
    fn slice(&mut self, new: Range<usize>) -> SliceOfList {
        let new_start = new.start + self.range.start;
        let new_end = new.end + self.range.start;
        assert!(new_end <= self.range.end);
        SliceOfList {
            range: new_start..new_end,
            list: &mut self.list,
        }
    }
    fn len(&self) -> usize {
        self.range.len()
    }
}

pub struct SortPlayer {
    starting_vec: Vec<usize>,
    record_of_operations: Vec<Operation>,
    length: usize,
    current_play_back_point: usize,
    playback_vec: Vec<usize>,
}

impl SortPlayer {
    pub fn new(length: usize, func: fn(&mut List)) -> Self {
        let input = starting(length);
        let mut list = List::new(input.clone(), length);
        func(&mut list);

        Self {
            starting_vec: input.clone(),
            record_of_operations: list.record_of_operations,
            length: list.length,
            playback_vec: input.clone(),
            current_play_back_point: 0,
        }
    }
    fn playback_complete(&self) -> bool {
        self.current_play_back_point == self.record_of_operations.len()
    }
    pub fn reset_play(&mut self) {
        self.playback_vec = self.starting_vec.clone();
        self.current_play_back_point = 0;
    }
    pub fn draw_state(&self, draw: &Draw) {
        let length = self.length as f32;
        for (i, x) in self.playback_vec.iter().enumerate() {
            let height = *x as f32 / length;
            let width = 1.0 / length;
            let offset_x = i as f32 / length;
            let (center, wh) = rect_corner_wh(Vec2::new(offset_x, 0.0), Vec2::new(width, height));
            draw.rect().xy(center).wh(wh).hsv(height, 0.8, 0.5);
        }
    }
    pub fn increment_playback(&mut self) {
        let next_op = self.record_of_operations[self.current_play_back_point];
        self.apply_op(next_op);
        self.current_play_back_point += 1;
    }
    pub fn play(&mut self, x: usize) {
        for _ in 0..x {
            if !self.playback_complete() {
                self.increment_playback();
            }
        }
    }
    fn apply_op(&mut self, op: Operation) {
        match op {
            Operation::Get(_x) => {}
            Operation::Set(i, x) => {
                self.playback_vec[i] = x;
            }
            Operation::Swap(a, b) => {
                self.playback_vec.swap(a, b);
            }
        }
    }
}

pub fn starting(length: usize) -> Vec<usize> {
    let mut v = (0..length)
        .into_iter()
        .map(|x| x + 1)
        .collect::<Vec<usize>>();
    v.shuffle(&mut thread_rng());
    v
}
