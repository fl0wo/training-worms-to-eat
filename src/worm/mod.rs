pub mod starve;
pub mod draw;
pub mod r#move;
pub mod generate;
pub mod search;

use raylib::color::Color;
use raylib::drawing::{RaylibDraw};
use raylib::math::{Vector2};

pub struct Worm {
    pub pos: Vector2, // in pixels
    pub prev_pos: Vector2, // in pixels
    pub dir: Vector2,
    pub color: Color, // constant
    pub rotation: f32, // in radians
    pub speed: f32, // from 0 to 1
    pub ray: f32, // constant
    pub life: f32, // from 0 to 1
}