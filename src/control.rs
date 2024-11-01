use raylib::camera::Camera2D;
use raylib::consts::MouseButton;
use raylib::drawing::{RaylibDrawHandle};
use raylib::math::Vector2;

const MOUSE_SENSITIVITY: f32 = 0.75;
const MOUSE_ZOOM_SENSITIVITY: f32 = 0.005;

const MAX_ZOOM: f32 = 3.0;
const MIN_ZOOM: f32 = 0.5;


pub fn handle_controls(
    p0: &mut Camera2D,
    d: &mut RaylibDrawHandle,
    prev_mouse_pos: &mut Vector2,
) {

    // mouse drag to move camera
    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let mouse_pos = d.get_mouse_position();
        let diff = Vector2::new(
            (mouse_pos.x - prev_mouse_pos.x) * MOUSE_SENSITIVITY,
            (mouse_pos.y - prev_mouse_pos.y) * MOUSE_SENSITIVITY,
        );

        p0.offset.x += diff.x;
        p0.offset.y += diff.y;

        *prev_mouse_pos = mouse_pos;
    } else {
        *prev_mouse_pos = d.get_mouse_position();
    }

    // if scroll, zoom in or out
    let scroll = d.get_mouse_wheel_move();

    if scroll != 0.0 {
        p0.zoom += scroll * MOUSE_ZOOM_SENSITIVITY;
        if p0.zoom > MAX_ZOOM {
            p0.zoom = MAX_ZOOM;
        } else if p0.zoom < MIN_ZOOM {
            p0.zoom = MIN_ZOOM;
        }
    }

    println!("camera offset: {:?}", p0.offset);
    println!("camera zoom: {:?}", p0.zoom);
}