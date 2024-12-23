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
    focus_target: &Option<Vector2>,
) {

    // mouse drag to move camera
    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
    } else {
        *prev_mouse_pos = d.get_mouse_position();
    }

    // if scroll, zoom in or out
    let scroll = d.get_mouse_wheel_move();

    if scroll != 0.0 {
        let mouse_pos = d.get_mouse_position();

        let offset = Vector2::new(
            (mouse_pos.x - p0.offset.x) / p0.zoom,
            (mouse_pos.y - p0.offset.y) / p0.zoom,
        );

        p0.offset.x -= offset.x * scroll * MOUSE_ZOOM_SENSITIVITY;
        p0.offset.y -= offset.y * scroll * MOUSE_ZOOM_SENSITIVITY;

        p0.zoom += scroll * MOUSE_ZOOM_SENSITIVITY;

        if p0.zoom > MAX_ZOOM {
            p0.zoom = MAX_ZOOM;
        } else if p0.zoom < MIN_ZOOM {
            p0.zoom = MIN_ZOOM;
        }
    }

    if let Some(target) = focus_target {
        p0.target = p0.target.lerp(
            Vector2::new(
                target.x - d.get_screen_width() as f32 / 2.0,
                target.y - d.get_screen_height() as f32 / 2.0,
            ),
            0.005,
        );
    }
}