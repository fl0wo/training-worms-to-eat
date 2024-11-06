use rand::Rng;
use raylib::math::Vector2;

pub fn rand_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn rand_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn add_vec2(a: Vector2, b: Vector2, mul: f32) -> Vector2 {
    Vector2::new(
        a.x + b.x * mul,
        a.y + b.y * mul,
    )
}

pub fn sub_vec2(a: Vector2, b: Vector2, mul: f32) -> Vector2 {
    Vector2::new(
        a.x - b.x * mul,
        a.y - b.y * mul,
    )
}

pub fn vector_between(a: Vector2, b: Vector2) -> Vector2 {
    Vector2::new(
        (a.x + b.x) / 2.0,
        (a.y + b.y) / 2.0
    )
}

pub fn from_angle_to_vec2(angle: f32) -> Vector2 {
    Vector2::new(
        angle.cos(),
        angle.sin(),
    )
}