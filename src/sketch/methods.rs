use crate::sketch::algorithms::{bubble, bucket, mergesort, quicksort, radix, selection};
use crate::sketch::player::SortPlayer;
use crate::sketch::{renderers, List, Model};
use nannou::Draw;
use std::fmt::Formatter;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum SortMethod {
    Quick,
    Merge,
    Bubble,
    Selection,
    Radix,
    Bucket,
}

impl SortMethod {
    pub fn func(&self) -> fn(&mut List) {
        match self {
            SortMethod::Quick => quicksort::sort,
            SortMethod::Merge => mergesort::sort,
            SortMethod::Bubble => bubble::sort,
            SortMethod::Selection => selection::sort,
            SortMethod::Radix => radix::sort,
            SortMethod::Bucket => bucket::sort,
        }
    }
    pub fn index(&self) -> usize {
        Self::iter().enumerate().find(|(_, x)| x == self).unwrap().0
    }
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} (F{})", self, self.index() + 1)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, EnumIter)]
pub enum RenderMethod {
    Classic,
    DisparityDots,
    ColourCircle,
    ColourTowers,
}

impl RenderMethod {
    pub fn func(&self) -> fn(player: &SortPlayer, draw: &Draw, aspect: f32) {
        match self {
            RenderMethod::Classic => renderers::classic::draw_state,
            RenderMethod::DisparityDots => renderers::disparity_dots::draw_state,
            RenderMethod::ColourCircle => renderers::colour_circle::draw_state,
            RenderMethod::ColourTowers => renderers::colour_towers::draw_state,
        }
    }
    pub fn draw(&self, player: &SortPlayer, draw: &Draw, aspect: f32) {
        self.func()(player, draw, aspect);
    }
    pub fn index(&self) -> usize {
        Self::iter().enumerate().find(|(_, x)| x == self).unwrap().0
    }
}

impl std::fmt::Display for RenderMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self, self.index() + 1)
    }
}

#[macro_export]
macro_rules! restart {
    ($x:expr) => {
        $x.player = SortPlayer::new(
            2_usize.pow($x.length_log2 as u32),
            $x.sorter.func(),
            $x.player.playback_rate,
            $x.reshuffle_on_change,
            $x.player.playback_vec.clone()
        )
    };
}

// pub fn restart(model: &mut Model) {
//     model.player = SortPlayer::new(
//         2_usize.pow(model.length_log2 as u32),
//         model.sorter.func(),
//         model.player.playback_rate,
//         model.reshuffle_on_change,
//         model.player.playback_vec.clone()
//     );
// }
