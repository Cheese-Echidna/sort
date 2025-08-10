use std::fmt::Formatter;
use nannou::Draw;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::sketch::algorithms::{bubble, bucket, mergesort, quicksort, radix, selection};
use crate::sketch::{renderers, List};
use crate::sketch::player::SortPlayer;

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
macro_rules! resort {
    ($x:expr) => {
        $x.player = SortPlayer::new(
            2_usize.pow($x.length_log2 as u32),
            $x.sorter.func(),
            $x.player.playback_rate
        )
    };
}