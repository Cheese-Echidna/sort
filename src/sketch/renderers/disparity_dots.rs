use std::f32::consts::TAU;
use nannou::Draw;
use nannou::geom::Vec2;
use crate::sketch::player::SortPlayer;


pub fn draw_state(player: &SortPlayer, draw: &Draw, aspect: f32) {
    let draw = draw.scale(0.5).xy(Vec2::splat(1.0));

    let length = player.length as f32;
    let gets = player.most_recent_gets();
    for (i, x) in player.playback_vec.iter().enumerate() {
        let height = (x + 1) as f32 / length;
        let i_prop = (i as f32 / length) * TAU;
        let dot_rad = (1.0 / length * 2.0).max(0.005);
        let pos = Vec2::new(i_prop.cos() / aspect, i_prop.sin()) * radius(i, *x, length) * 0.9;

        let q = gets.as_ref().map(|x| x.get(&i).cloned()).flatten().unwrap_or(0.);
        let (s, v) = player.sv(q);
        let (h, s, v) = (height, s, v);
        draw.ellipse().resolution(100.).xy(pos).wh(Vec2::new(dot_rad, dot_rad * aspect)).hsv(h, s, v);
    }
}

pub(crate) fn radius(index: usize, value: usize, length: f32) -> f32 {
    let i_prop = index as f32 / length;
    let v_prop = value as f32 / length;
    let diff = (i_prop - v_prop).abs();
    let act_diff = diff.min(1.0 - diff);
    let radius = 1.0 - act_diff * 2.0;
    radius
}