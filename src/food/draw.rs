use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};
use crate::food::{Food, ray_of_food};
pub fn draw_foods(
    d2d: &mut RaylibMode2D<RaylibDrawHandle>,
    foods: &Vec<Food>,
    percentage_animation: f32 // from 0 to 1
) {
    for food in foods.iter() {
        draw_food(
            d2d,
            food,
            percentage_animation
        );
    }
}

fn draw_food(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    food: &Food,
    percentage_animation: f32 // from 0 to 1
) {
    d.draw_circle(
        food.pos.x as i32,
        food.pos.y as i32,
        ray_of_food(food),
        food.color
    );
}
