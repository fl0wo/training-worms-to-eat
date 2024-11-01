use raylib::math::Vector2;
use crate::food::Food;
use crate::math::{rand_float, rand_int};

pub fn generate_food(num_food: i32) -> Vec<Food> {

    let mut food = Vec::new();

    for _ in 0..num_food {
        let pos = Vector2::new(
            rand_int(0, 800) as f32,
            rand_int(0, 450) as f32
        );

        let amount = rand_float(0.0, 100.0);

        food.push(Food {
            pos,
            amount,
            ray: 0.1 * amount,
            color: raylib::color::Color::new(255, 255, 255, 255)
        });
    }

    food
}