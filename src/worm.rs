use raylib::color::Color;
use raylib::math::Vector2;
use crate::math::add_vec2;

pub struct Worm {
    pub pos: Vector2,
    pub prev_pos: Vector2,
    pub dir: Vector2,
    pub color: Color,
    pub rotation: f32,
    pub speed: f32,
}

pub fn move_worm(worm: &mut Worm) {
    worm.prev_pos = worm.pos; // is using copy

    worm.pos.x += worm.dir.x * worm.speed;
    worm.pos.y += worm.dir.y * worm.speed;
    add_vec2(worm.pos, worm.dir);

    // adapt the rotation to look in the direction of the movement
    worm.rotation = worm.rotation.atan2(worm.dir.x);
}

pub fn move_worms(worms: &mut Vec<Worm>) {
    for worm in worms.iter_mut() {
        move_worm(worm);
    }
}