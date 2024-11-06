mod math;
mod map;
mod control;
mod worm;
mod food;
mod train;

use std::fs;
use std::ops::{Add, Deref, Sub};
use raylib::camera::Camera2D;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibMode2D, RaylibMode2DExt};
use raylib::math::Vector2;
use crate::control::handle_controls;
use crate::food::draw::draw_foods;
use crate::food::Food;
use crate::food::generate::generate_food;
use crate::map::draw_background;
use crate::math::from_angle_to_vec2;
use crate::train::WormModel;
use crate::worm::{Worm};
use crate::worm::draw::{draw_worms};
use crate::worm::generate::generate_worms;
use crate::worm::r#move::move_worms;
use crate::worm::search::highlight_selected_worm;
use crate::worm::starve::{feed_worms, starve_worms};

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
    let mut worms: Vec<Worm> = generate_worms(num_worms);
    let mut food: Vec<Food> = generate_food(10);

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

        draw_foods(
            &mut d2d,
            &food,
            (delta_time / EASING_SEC) as f32
        );

        draw_worms(
            &mut d2d,
            &worms,
            (delta_time / EASING_SEC) as f32
        );

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
            change_worms_direction(&mut worms, &food);
            move_worms(&mut worms);
            starve_worms(&mut worms);
            feed_worms(&mut worms, &mut food);
        }

    }
}

fn change_worms_direction(worms: &mut Vec<Worm>, foods: &Vec<Food>) {
    // Load and parse the model only once
    let model_json = match fs::read_to_string("worm_model.json") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading model file: {}", e);
            return;
        }
    };

    let model: WormModel = match serde_json::from_str(&model_json) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error parsing model JSON: {}", e);
            return;
        }
    };

    let foods_models = foods.iter().map(|food| {
        train::Food {
            position: (food.pos.x as i32, food.pos.y as i32),
            value: food.amount as i32,
        }
    }).collect::<Vec<_>>();

    // Update each worm's direction using the model's brain
    for worm in worms.iter_mut() {
        let direction = model.brain.calculate_direction(
            (worm.pos.x as f64, worm.pos.y as f64),
            foods_models.deref(),
        );

        // Store the calculated direction (in radians) in the worm
        worm.dir = from_angle_to_vec2(direction as f32);

        // If the worm's life is below threshold, increase speed
        if worm.life < model.brain.life_threshold as f32 {
            worm.speed = (model.brain.speed_factor * 1.5) as f32;
        } else {
            worm.speed = model.brain.speed_factor as f32;
        }
    }
}