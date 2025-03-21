use nannou::Draw;
use nannou::geom::Vec2;
use crate::player::SortPlayer;

pub fn draw_state(player: &SortPlayer, draw: &Draw, _aspect: f32) {
    let length = player.length as f32;
    let gets = player.most_recent_gets(25);
    for (i, x) in player.playback_vec.iter().enumerate() {
        let height = (x + 1) as f32 / length;
        let width = 1.0 / length;
        let offset_x = i as f32 / length;
        let (center, wh) = rect_corner_wh(Vec2::new(offset_x, 0.0), Vec2::new(width, height));
        let v = 0.5 + gets.get(&i).unwrap_or(&0.) * 0.5;
        draw.rect().xy(center).wh(wh).hsv(height, 0.8, v);
    }
}

fn rect_corner_wh(bottom_left: Vec2, wh: Vec2) -> (Vec2, Vec2) {
    let center = bottom_left + wh / 2.;
    // let width = (corner1 - corner2).abs();
    (center, wh)
}