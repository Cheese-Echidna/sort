use nannou::Draw;
use nannou::draw::primitive::polygon::PolygonOptions;
use nannou::geom::Vec2;
use crate::sketch::player::SortPlayer;

pub fn draw_state(player: &SortPlayer, draw: &Draw, _aspect: f32) {
    let length = player.length as f32;
    let fudge_factor = length;
    let draw = draw.scale(1.0 / fudge_factor);

    let gets = player.most_recent_gets();
    for (i, &x) in player.playback_vec.iter().enumerate() {
        let height = x as f32 / length;
        let width = 1.0 / length;
        let offset_x = i as f32 / length;
        let points = four_corners_trap(Vec2::new(offset_x, 0.0), Vec2::new(width, height), 1.0 / length);
        let q = gets.as_ref().map(|x| x.get(&i).cloned()).flatten().unwrap_or(0.);
        let (s, v) = player.sv(q);
        draw.polygon().points(points.into_iter().map(|x| x * fudge_factor).collect::<Vec<_>>()).hsv(height, s, v);
    }
}

fn _rect_corner_wh(bottom_left: Vec2, wh: Vec2) -> (Vec2, Vec2) {
    let center = bottom_left + wh / 2.;
    // let width = (corner1 - corner2).abs();
    (center, wh)
}

fn _four_corners_rect(bottom_left: Vec2, wh: Vec2) -> [Vec2; 4] {
    let top_right = bottom_left + wh;
    let bottom_right = bottom_left + wh * Vec2::X;
    let top_left = bottom_left + wh * Vec2::Y;
    [bottom_left, bottom_right, top_right, top_left]
}

fn four_corners_trap(bottom_left: Vec2, wh: Vec2, dh: f32) -> Vec<Vec2> {
    let top_right = bottom_left + wh + Vec2::new(0.0, dh);
    let bottom_right = bottom_left + wh * Vec2::X;
    let top_left = bottom_left + wh * Vec2::Y;
    vec![bottom_left, bottom_right, top_right, top_left]
}
