use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};
use crate::food::Food;

pub fn draw_food(
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
