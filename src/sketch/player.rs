use crate::sketch::list::Operation;
use crate::sketch::{shuffle_step_by_step, zing, List};
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Audio {
    pub(crate) phase: f64,
    pub(crate) hz: f64,
    pub volume: f64,
}

fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = audio.volume;

    for frame in buffer.frames_mut() {
        let t = audio.phase % 1.0;

        let triangle = true;

        let sample = if triangle {
            // Triangle wave
            4.0 * (t - 0.5).abs() - 1.0
        } else {
            // Sine wave
            (2.0 * std::f64::consts::PI * audio.phase).sin()
        };

        audio.phase += audio.hz / sample_rate;
        if audio.phase >= 1.0 {
            audio.phase -= 1.0;
        }

        for channel in frame {
            *channel = (sample * volume) as f32;
        }
    }
}
pub struct SortPlayer {
    starting_vec: Vec<usize>,
    pub(crate) record_of_operations: Vec<Operation>,
    pub(crate) length: usize,
    current_play_back_point: usize,
    pub(crate) playback_vec: Vec<usize>,
    pub(crate) playback_rate: usize,
    pub(crate) stream: audio::Stream<Audio>,
}

impl SortPlayer {
    pub fn new(
        length: usize,
        sort: fn(&mut List),
        speed: usize,
        shuffle: bool,
        list: Vec<usize>,
    ) -> Self {
        let length_equal = length == list.len();

        let input = if length_equal && contains_all(&list) {
            list
        } else {
            starting(length) // refresh
        };
        let mut list = List::new(input.clone(), length);

        if shuffle || !length_equal {
            shuffle_step_by_step(&mut list);
        }

        if !list.is_sorted() {
            sort(&mut list);
        }
        zing(&mut list);

        let audio_model = Audio {
            phase: 0.0,
            hz: 440.0,
            volume: 0.2,
        };

        let audio_host = audio::Host::new();
        let stream = audio_host
            .new_output_stream(audio_model)
            .render(audio)
            .build()
            .unwrap();

        stream.play().unwrap();

        Self {
            starting_vec: input.clone(),
            record_of_operations: list.record_of_operations,
            length,
            playback_vec: input.clone(),
            current_play_back_point: 0,
            playback_rate: speed,
            stream,
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
        let v = match next_op {
            Operation::Get(i) => self.playback_vec[i],
            Operation::Set(i, v) => v,
            Operation::Swap(i, j) => self.playback_vec[i],
        };
        let x = lerp(120.0, 1212.0, v as f64 / self.length as f64);

        self.stream.play().unwrap();

        self.stream
            .send(move |audio| {
                audio.hz = x;
            })
            .unwrap();
    }
    pub fn play(&mut self, x: usize) {
        for _ in 0..x {
            if !self.playback_complete() {
                self.increment_playback();
            } else {
                self.stream.pause().unwrap();
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
                map.entry(a)
                    .and_modify(|x: &mut f32| *x = x.max(prop))
                    .or_insert(prop);
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
    let v = (0..length).into_iter().map(|x| x).collect::<Vec<usize>>();
    // v.shuffle(&mut thread_rng());
    v
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

pub fn contains_all(l: &Vec<usize>) -> bool {
    let mut v = l.clone();
    v.sort();
    v.windows(2).all(|x| x[0] == x[1] - 1)
}
