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

pub fn add_vec2(a: Vector2, b: Vector2) -> Vector2 {
    Vector2::new(a.x + b.x, a.y + b.y)
}

pub fn vector_between(a: Vector2, b: Vector2) -> Vector2 {
    Vector2::new(
        (a.x + b.x) / 2.0,
        (a.y + b.y) / 2.0
    )
}