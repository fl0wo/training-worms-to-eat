use raylib::color::Color;
use raylib::math::Vector2;
use crate::math::{rand_float, rand_int};
use crate::{HEIGHT, WIDTH};
use crate::worm::Worm;

pub fn generate_worms(num_worms: i32) -> Vec<Worm> {
    let mut worms = Vec::new();

    for _ in 0..num_worms {
        // worms becomes the owner of the Worm struct
        // can only be 1 owner at a time

        // when passing with reference, can pass infinite const immutable (readonly) references
        // but max 1 mutable reference at a time

        let initial_pos = Vector2::new(
            rand_int(0, WIDTH) as f32,
            rand_int(0, HEIGHT) as f32
        );

        worms.push(Worm {
            prev_pos: initial_pos,
            pos: initial_pos,
            dir: Vector2::new(
                rand_float(-1.0, 1.0) as f32,
                rand_float(-1.0, 1.0) as f32
            ),
            color: Color::new(85, 239, 196, 255),
            speed: rand_float(10.0, 30.0),
            rotation: 0.0,
            ray: 10.0,
            life: 1.0,
        });
    }

    worms
}
