pub mod draw;
pub mod generate;

use raylib::color::Color;
use raylib::math::Vector2;

pub struct Food {
    pub pos: Vector2,
    pub color: Color,
    pub ray: f32,
    pub amount: f32, // from 1 to 100
}