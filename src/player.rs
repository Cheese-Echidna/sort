use std::collections::HashMap;
use nannou::rand::prelude::SliceRandom;
use nannou::rand::thread_rng;
use crate::list::Operation;
use crate::List;

pub struct SortPlayer {
    starting_vec: Vec<usize>,
    pub(crate) record_of_operations: Vec<Operation>,
    pub(crate) length: usize,
    current_play_back_point: usize,
    pub(crate) playback_vec: Vec<usize>,
    pub(crate) playback_rate: usize,
}

impl SortPlayer {
    pub fn new(length: usize, sort: fn(&mut List), speed: usize) -> Self {
        let input = starting(length);
        let mut list = List::new(input.clone(), length);
        sort(&mut list);

        Self {
            starting_vec: input.clone(),
            record_of_operations: list.record_of_operations,
            length,
            playback_vec: input.clone(),
            current_play_back_point: 0,
            playback_rate: speed,
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
    pub(crate) fn most_recent_gets(&self) -> Option<HashMap<usize, f32>> {
        let history_dist = (self.length / 20).max(1).min(self.current_play_back_point);
        if self.playback_complete() || self.current_play_back_point == 0 {
            return None;
        }
        let mut map = HashMap::new();
        for j in 0..history_dist {
            let i = self.current_play_back_point - j - 1;
            let prop = if history_dist == 1 {
                1.0
            } else {
                1.0 - (j as f32 / (history_dist - 1) as f32)
            };
            if let Operation::Get(a) = self.record_of_operations[i] {
                map.entry(a).and_modify(|x: &mut f32| *x = x.max(prop)).or_insert(prop);
            }
        }
        Some(map)
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
    pub(crate) fn sv(&self, q: f32) -> (f32, f32) {
        let v = 0.5 + 0.5 * q;
        let s = 0.8 - 0.2 * q;
        (s, v)
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
