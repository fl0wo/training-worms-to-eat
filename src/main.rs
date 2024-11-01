mod math;
mod map;
mod control;
mod worm;


use std::ops::{Add, Deref, Sub};
use raylib::camera::Camera2D;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D, RaylibMode2DExt};
use raylib::math::Vector2;
use crate::control::handle_controls;
use crate::map::draw_background;
use crate::math::{add_vec2, rand_float, rand_int, sub_vec2};
use crate::worm::{Worm};
use crate::worm::draw::draw_worm;
use crate::worm::r#move::move_worms;
use crate::worm::starve::starve_worms;


const EASING_SEC: f64 = 0.5;
const WIDTH: i32 = 800;
const HEIGHT: i32 = 800;

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

    let mut prev_time = rl.get_time();

    // handle mousewheel to zoom in and out
    let mut camera = Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut prev_mouse_pos = Vector2::zero();
    let mut focus_target: Option<Vector2> = None;

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        handle_controls(
            &mut camera,
            &mut d,
            &mut prev_mouse_pos,
            &focus_target,
        );
        let mut d2d = d.begin_mode2D(camera);

        d2d.clear_background(Color::new(45, 52, 54, 255));
        draw_background(&mut d2d);

        let current_time = d2d.get_time();
        let delta_time = current_time - prev_time;

        for worm in worms.iter() {
            draw_worm(
                &mut d2d,
                worm,
                (delta_time / EASING_SEC) as f32
            );
        }

        if let Some(worm) = highlight_selected_worm(
            &worms,
            &mut d2d,
            &camera
        ) {
            focus_target = Some(worm.pos);
        }

        // if delta time is succeeded, move the worms
        if(current_time - prev_time > EASING_SEC) {
            prev_time = current_time;
            move_worms(&mut worms);
            starve_worms(&mut worms);
        }

    }
}

fn highlight_selected_worm<'a>(
    worms: &'a Vec<Worm>,
    d2d: &mut RaylibMode2D<RaylibDrawHandle>,
    camera: &Camera2D
) -> Option<&'a Worm> {

    let cur_mouse_pos = d2d.get_screen_to_world2D(
        d2d.get_mouse_position(),
        camera
    );

    let closest_worm = worms
        .iter()
        .min_by(|a, b| {
            a.pos.distance_to(cur_mouse_pos).partial_cmp(&b.pos.distance_to(cur_mouse_pos)).unwrap()
        });

    match closest_worm {
        Some(worm) => {
            d2d.draw_circle_lines(
                worm.pos.x as i32,
                worm.pos.y as i32,
                worm.ray * 4.0 + 2.0,
                Color::WHITE
            );
        },
        None => {}
    }

    return closest_worm
}
