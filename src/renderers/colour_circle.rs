use std::f32::consts::TAU;
use nannou::Draw;
use nannou::geom::Vec2;
use crate::player::SortPlayer;

pub fn draw_state(player: &SortPlayer, draw: &Draw, aspect: f32) {
    let draw = draw.scale(0.5).xy(Vec2::splat(1.0));

    let length = player.length as f32;
    let gets = player.most_recent_gets();
    for (i, x) in player.playback_vec.iter().enumerate() {
        let height = (x + 1) as f32 / length;
        let i_prop = (i as f32 / length) * TAU;
        let i_next_prop = ((i as f32 + 1.0) / length) * TAU;

        let pos = Vec2::new(i_prop.cos() / aspect, i_prop.sin()) * 0.9;
        let pos_next = Vec2::new(i_next_prop.cos() / aspect, i_next_prop.sin()) * 0.9;

        let q = gets.get(&i).cloned().unwrap_or(0.);
        let (s, v) = player.sv(q);
        let (h, s, v) = (height, s, v);
        draw.polygon().points([pos, pos_next, Vec2::ZERO]).hsv(h, s, v);
    }
}

