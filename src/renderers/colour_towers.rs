use std::f32::consts::TAU;
use nannou::Draw;
use nannou::geom::Vec2;
use crate::player::SortPlayer;
use crate::renderers::disparity_dots::radius;

pub fn draw_state(player: &SortPlayer, draw: &Draw, aspect: f32) {
    let draw = draw.scale(0.5).xy(Vec2::splat(1.0));

    let length = player.length as f32;
    let gets = player.most_recent_gets();
    for (i, x) in player.playback_vec.iter().enumerate() {
        let height = (x + 1) as f32 / length;
        let i_prop = (i as f32 / length) * TAU;
        let i_next_prop = ((i as f32 + 1.0) / length) * TAU;
        // let dot_rad = 1.0 / length * 2.0;
        let radius = radius(i, *x, length);
        // let radius = 1.0;
        let pos = Vec2::new(i_prop.cos() / aspect, i_prop.sin()) * radius * 0.9;
        let pos_next = Vec2::new(i_next_prop.cos() / aspect, i_next_prop.sin()) * radius * 0.9;

        let q = gets.as_ref().map(|x| x.get(&i).cloned()).flatten().unwrap_or(0.);
        let (s, v) = player.sv(q);
        let (h, s, v) = (height, s, v);
        draw.polygon().points([pos, pos_next, Vec2::ZERO]).hsv(h, s, v);
    }
}
