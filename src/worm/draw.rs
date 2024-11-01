use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D};
use raylib::math::{Rectangle, Vector2};
use crate::math::vector_between;
use crate::worm::Worm;


pub fn draw_worms(
    d2d: &mut RaylibMode2D<RaylibDrawHandle>,
    worms: &Vec<Worm>,
    percentage_animation: f32 // from 0 to 1
) {
    for worm in worms.iter() {
        draw_worm(
            d2d,
            worm,
            percentage_animation
        );
    }
}

// 0% => a circle in the previous position
// 50% => a circle in the final position (and a rectangle following the circle with the right rotation)
// 100% => a circle in the final position
fn draw_worm(
    d: &mut RaylibMode2D<RaylibDrawHandle>,
    worm: &Worm,
    percentage_animation: f32 // from 0 to 1
) {
    let circle_tail = worm.prev_pos.lerp(
        worm.pos,
        0f32.max(percentage_animation - 0.5) * 2.0 // starts after 50% and goes to 100%
    );

    let circle_head = worm.prev_pos.lerp(
        worm.pos,
        1f32.min(percentage_animation * 2.0) // 2x faster to reach 100%
    );

    let color_head = worm.color;
    let color_tail = worm.color.brightness(-0.2);

    let body_center = vector_between(
        circle_tail,
        circle_head
    );

    let body_length = circle_tail.distance_to(circle_head);
    let body_height = worm.ray * 2.0;

    d.draw_circle(
        circle_head.x as i32,
        circle_head.y as i32,
        worm.ray,
        color_head
    );

    d.draw_circle(
        circle_tail.x as i32,
        circle_tail.y as i32,
        worm.ray,
        color_tail
    );

    d.draw_rectangle_pro(
        Rectangle::new(body_center.x, body_center.y, body_length, body_height),
        Vector2::new(body_length/2.0, body_height/2.0),
        worm.rotation.to_degrees(),
        color_tail
    );

    let life = worm.life // from 0 to 1
        .max(0.0)
        .min(1.0);

    let life_color = Color::new(
        (255.0 * (1.0 - life)) as u8,
        (255.0 * (0.0 + life)) as u8,
        0,
        255
    );

    let life_bar_width:f32 = 30.0;

    d.draw_rectangle(
        (circle_head.x - (life_bar_width/2.0)) as i32,
        circle_head.y as i32 - (worm.ray * 2.0) as i32,
        (life_bar_width * life) as i32,
        5,
        life_color
    );
}
