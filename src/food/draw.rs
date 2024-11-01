use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};
use crate::food::Food;
use crate::math::{rand_float, rand_int};

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
    let normalized_amount = food.amount / 100.0; // from 0 to 1

    for i in 0..(normalized_amount * 10.0) as i32 {
        let radius = food.ray * (i as f32 + 1.0) * 0.1;
        let alpha = normalized_amount * 0.5 + 0.5;

        d.draw_circle(
            food.pos.x as i32,
            food.pos.y as i32,
            radius,
            food.color.fade(alpha)
        );
    }

}
