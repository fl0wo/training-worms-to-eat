use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use crate::math::{add_vec2, vector_between};

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

pub fn move_worm(worm: &mut Worm) {
    worm.prev_pos = worm.pos; // is using copy

    worm.pos.x += worm.dir.x * worm.speed;
    worm.pos.y += worm.dir.y * worm.speed;

    add_vec2(worm.pos, worm.dir);

    // adapt the rotation to look in the direction of the movement
    worm.rotation = worm.dir.y.atan2(worm.dir.x);
}

pub fn move_worms(worms: &mut Vec<Worm>) {
    for worm in worms.iter_mut() {
        move_worm(worm);
    }
}

/**
Based on how fast the worms are moving, they will lose energy.
*/
pub fn starve_worms(worms: &mut Vec<Worm>) {

    let factor = 0.001;

    for worm in worms.iter_mut() {
        worm.life -= (factor + (factor * worm.speed));
    }

    worms.retain(|worm| worm.life > 0.0);
}

// 0% => a circle in the previous position
// 50% => a circle in the final position (and a rectangle following the circle with the right rotation)
// 100% => a circle in the final position
pub fn draw_worm(
    d: &mut RaylibDrawHandle,
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