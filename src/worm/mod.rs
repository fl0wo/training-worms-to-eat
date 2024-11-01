pub mod starve;
pub mod draw;
pub mod r#move;

use raylib::color::Color;
use raylib::drawing::{RaylibDraw};
use raylib::math::{Vector2};

pub struct Worm {
    pub pos: Vector2,
    pub prev_pos: Vector2,
    pub dir: Vector2,
    pub color: Color,
    pub rotation: f32,
    pub speed: f32,
    pub ray: f32,
    pub life: f32,
}