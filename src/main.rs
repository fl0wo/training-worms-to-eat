mod math;
mod worm;

use std::ops::Deref;
use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
/**
We generate 100 worms that move around randomly on the screen.
 */

use crate::math::{rand_float, rand_int};
use crate::worm::{move_worm, move_worms, Worm};

const EASING_SEC: f32 = 0.5;

fn main()
{
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Training Worms")
        .vsync()
        .build();

    let num_worms = 100;

    // mutable means that the value of worms can be changed
    let mut worms: Vec<Worm> = Vec::new();

    for _ in 0..num_worms {
        // worms becomes the owner of the Worm struct
        // can only be 1 owner at a time

        // when passing with reference, can pass infinite const immutable (readonly) references
        // but max 1 mutable reference at a time

        let initial_pos = Vector2::new(
            rand_int(0, 800) as f32,
            rand_int(0, 800) as f32
        );

        worms.push(Worm {
            prev_pos: initial_pos,
            pos: initial_pos,
            dir: Vector2::new(
                rand_float(-1.0, 1.0) as f32,
                rand_float(-1.0, 1.0) as f32
            ),
            color: Color::YELLOW,
            speed: 20.0,
            rotation: 0.0,
        });
    }

    let mut delta_time:f64 = rl.get_time();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for worm in worms.iter() {

            // instead of just drawing em, see % of frames elapsed till next move,
            // to calculate in which frame of the animation the worm should be

            let pos_lerp = worm.prev_pos.lerp(
                worm.pos,
                ((d.get_time() - delta_time) as f32) / EASING_SEC
            );

            d.draw_circle(
                pos_lerp.x as i32,
                pos_lerp.y as i32,
                4.0,
                worm.color
            );
        }

        // if delta time is succeded, move the worms
        if(d.get_time() - delta_time > EASING_SEC as f64) {
            delta_time = d.get_time();
            move_worms(&mut worms);
        }
    }
}