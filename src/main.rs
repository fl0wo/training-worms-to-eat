mod math;
mod worm;

use std::ops::Deref;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
/**
We generate 100 worms that move around randomly on the screen.
 */

use crate::math::{rand_float, rand_int};
use crate::worm::{draw_worm, move_worm, move_worms, Worm};

const EASING_SEC: f64 = 0.5;

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
            color: Color::new(85, 239, 196, 255),
            speed: 20.0,
            rotation: 0.0,
            ray: 10.0,
        });
    }

    let mut prev_time = rl.get_time();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(45, 52, 54, 255));
        draw_grid_lines(&mut d);

        let current_time = d.get_time();
        let delta_time = current_time - prev_time;

        for worm in worms.iter() {
            draw_worm(
                &mut d,
                worm,
                (delta_time / EASING_SEC) as f32
            );
        }

        // if delta time is succeded, move the worms
        if(d.get_time() - prev_time > EASING_SEC as f64) {
            prev_time = d.get_time();
            move_worms(&mut worms);
        }


    }
}

fn draw_grid_lines(d: &mut RaylibDrawHandle) {
    // draw 10 hor and 10 ver lines (color rgb(170, 166, 157))
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    let n_lines = 10;

    let line_color = Color::new(170, 166, 157, 50);

    for i in 0..n_lines {
        let x = (w / n_lines) * i;
        let y = (h / n_lines) * i;

        d.draw_line(x, 0, x, h, line_color);
        d.draw_line(0, y, w, y, line_color);
    }
}