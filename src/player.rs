use std::collections::HashMap;
use nannou::rand::prelude::SliceRandom;
use nannou::rand::thread_rng;
use crate::list::Operation;
use crate::List;

pub struct SortPlayer {
    starting_vec: Vec<usize>,
    record_of_operations: Vec<Operation>,
    pub(crate) length: usize,
    current_play_back_point: usize,
    pub(crate) playback_vec: Vec<usize>,
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
    pub(crate) fn most_recent_gets(&self, history_dist: usize) -> HashMap<usize, f32> {
        if self.playback_complete() {
            return HashMap::new();
        }
        let mut map = HashMap::new();
        self.record_of_operations[..self.current_play_back_point].iter().enumerate().filter(|(i, op)| {
            (self.current_play_back_point - 1 - i) <= history_dist
        }).filter_map(|(i, op)| {
            match op {
                Operation::Get(x) => {Some((*x, 1.0 - (self.current_play_back_point - i) as f32 / history_dist as f32))}
                Operation::Set(_, _) => {None}
                Operation::Swap(_, _) => {None}
            }
        }).for_each(|(i, t)| {
            map.entry(i).and_modify(|x: &mut f32| *x = x.min(t)).or_insert(t);
        });
        map
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
        .map(|x| x)
        .collect::<Vec<usize>>();
    v.shuffle(&mut thread_rng());
    v
}
