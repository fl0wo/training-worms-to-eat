use raylib::camera::Camera2D;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};
use crate::worm::Worm;

pub fn highlight_selected_worm<'a>(
    worms: &'a Vec<Worm>,
    d2d: &mut RaylibMode2D<RaylibDrawHandle>,
    camera: &Camera2D
) -> Option<&'a Worm> {

    let cur_mouse_pos = d2d.get_screen_to_world2D(
        d2d.get_mouse_position(),
        camera
    );

    let closest_worm = worms
        .iter()
        .min_by(|a, b| {
            a.pos.distance_to(cur_mouse_pos).partial_cmp(&b.pos.distance_to(cur_mouse_pos)).unwrap()
        });

    match closest_worm {
        Some(worm) => {
            d2d.draw_circle_lines(
                worm.pos.x as i32,
                worm.pos.y as i32,
                worm.ray * 4.0 + 2.0,
                Color::WHITE
            );
        },
        None => {}
    }

    return closest_worm
}
