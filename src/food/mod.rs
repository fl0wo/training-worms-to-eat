pub mod draw;
pub mod generate;

use raylib::color::Color;
use raylib::math::Vector2;

pub struct Food {
    pub pos: Vector2,
    pub color: Color,
    pub amount: f32, // from 1 to 100
}

pub fn ray_of_food(food: &Food) -> f32 {
    10.0 * food.amount.sqrt()
}