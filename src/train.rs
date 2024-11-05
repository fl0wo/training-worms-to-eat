use genevo::{
    operator::prelude::*,
    population::*,
    prelude::*,
    types::fmt::Display,
};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, Write};
use crate::math::rand_int;

#[derive(Debug, Clone)]
struct Food {
    position: (i32, i32),
    value: i32,
}

#[derive(Debug, Clone)]
struct Worm {
    position: (f64, f64),
    life: f64,
    food_consumed: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WormBrain {
    food_attraction: f64,
    speed_factor: f64,
    life_threshold: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct WormModel {
    brain: WormBrain,
    fitness: i32,
}

fn load_model(filename: &str) -> io::Result<WormModel> {
    let file = File::open(filename)?;
    let model = serde_json::from_reader(file)?;
    Ok(model)
}


fn save_model(model: &WormModel, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    serde_json::to_writer(file, model)?;
    Ok(())
}
impl WormBrain {
    fn calculate_direction(&self, worm_pos: (f64, f64), foods: &[Food]) -> f64 {
        if foods.is_empty() {
            // When no food is available, move in a circular pattern
            let current_angle = worm_pos.1.atan2(worm_pos.0);
            return (current_angle + 0.1) % (2.0 * std::f64::consts::PI);
        }

        // Find the closest valuable food
        let mut weighted_direction = 0.0;
        let mut total_weight = 0.0;

        for food in foods {
            let dx = food.position.0 as f64 - worm_pos.0;
            let dy = food.position.1 as f64 - worm_pos.1;
            let distance = (dx * dx + dy * dy).sqrt();

            // Weight based on distance and food value
            let weight = (food.value as f64) / (distance + 1.0);
            let direction = dy.atan2(dx);

            weighted_direction += direction * weight;
            total_weight += weight;
        }

        let base_angle = if total_weight == 0.0 {
            // If weights sum to zero, use current direction
            worm_pos.1.atan2(worm_pos.0)
        } else {
            weighted_direction / total_weight
        };

        // Add small fixed randomness instead of variable range
        let random_offset = rand::thread_rng().gen::<f64>() * 0.2 * (1.0 - self.food_attraction);

        // Ensure the result stays within [0, 2π]
        (base_angle + random_offset) % (2.0 * std::f64::consts::PI)
    }
}

#[test]
fn train() {
    let initial_foods: Vec<Food> = (0..10)
        .map(|_| Food {
            position: (rand_int(0, 100), rand_int(0, 100)),
            value: rand_int(2, 10),
        })
        .collect();

    let population_size = 50;
    let mut population: Vec<WormBrain> = (0..population_size)
        .map(|_| WormBrain {
            food_attraction: rand::thread_rng().gen_range(0.5..1.0),
            speed_factor: rand::thread_rng().gen_range(1.0..3.0),
            life_threshold: rand::thread_rng().gen_range(0.2..0.8),
        })
        .collect();

    let generations = 50;
    let mut best_brain = population[0].clone();
    let mut best_fitness = 0;

    for generation in 0..generations {
        let mut generation_fitness = Vec::new();

        for (i, brain) in population.iter().enumerate() {
            let fitness = evaluate_brain(brain, &initial_foods);
            generation_fitness.push((i, fitness));
        }

        generation_fitness.sort_by_key(|&(_, fitness)| std::cmp::Reverse(fitness));

        if generation_fitness[0].1 > best_fitness {
            best_fitness = generation_fitness[0].1;
            best_brain = population[generation_fitness[0].0].clone();
            println!("New best fitness: {} in generation {}", best_fitness, generation);
        }

        // Select top performers
        let top_performers: Vec<&WormBrain> = generation_fitness.iter()
            .take(population_size / 4)
            .map(|&(i, _)| &population[i])
            .collect();

        // Create new population
        let mut new_population = Vec::new();

        // Keep the best performer
        new_population.push(best_brain.clone());

        while new_population.len() < population_size {
            let parent1 = &top_performers[rand::thread_rng().gen_range(0..top_performers.len())];
            let parent2 = &top_performers[rand::thread_rng().gen_range(0..top_performers.len())];

            let mut child = crossover(parent1, parent2);
            mutate(&mut child);
            new_population.push(child);
        }

        population = new_population;
    }

    let model = WormModel {
        brain: best_brain,
        fitness: best_fitness,
    };

    if let Err(e) = save_model(&model, "worm_model.json") {
        eprintln!("Failed to save model: {}", e);
    } else {
        println!("Model saved successfully with fitness: {}", best_fitness);
    }
}

fn evaluate_brain(brain: &WormBrain, foods: &[Food]) -> i32 {
    let mut worm = Worm {
        position: (50.0, 50.0),
        life: 1.0,
        food_consumed: 0,
    };

    let mut available_foods = foods.to_vec();
    let iterations = 500;

    for _ in 0..iterations {
        if worm.life <= 0.0 {
            break;
        }

        let direction = brain.calculate_direction(worm.position, &available_foods);
        let speed = brain.speed_factor;

        // Update position with boundary checking
        let new_x = worm.position.0 + direction.cos() * speed;
        let new_y = worm.position.1 + direction.sin() * speed;
        worm.position.0 = new_x.max(0.0).min(100.0);
        worm.position.1 = new_y.max(0.0).min(100.0);

        available_foods.retain(|food| {
            let distance = ((worm.position.0 - food.position.0 as f64).powi(2)
                + (worm.position.1 - food.position.1 as f64).powi(2)).sqrt();

            if distance < 2.0 {
                worm.food_consumed += food.value;
                worm.life = (worm.life + 0.2).min(1.0);
                false
            } else {
                true
            }
        });

        worm.life -= 0.001 * speed;
    }

    worm.food_consumed
}

fn crossover(parent1: &WormBrain, parent2: &WormBrain) -> WormBrain {
    let mut rng = rand::thread_rng();
    WormBrain {
        food_attraction: if rng.gen_bool(0.5) { parent1.food_attraction } else { parent2.food_attraction },
        speed_factor: if rng.gen_bool(0.5) { parent1.speed_factor } else { parent2.speed_factor },
        life_threshold: if rng.gen_bool(0.5) { parent1.life_threshold } else { parent2.life_threshold },
    }
}

fn mutate(brain: &mut WormBrain) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.2) {
        brain.food_attraction += rng.gen_range(-0.1..0.1);
        brain.food_attraction = brain.food_attraction.clamp(0.0, 1.0);
    }
    if rng.gen_bool(0.2) {
        brain.speed_factor += rng.gen_range(-0.2..0.2);
        brain.speed_factor = brain.speed_factor.clamp(1.0, 3.0);
    }
    if rng.gen_bool(0.2) {
        brain.life_threshold += rng.gen_range(-0.1..0.1);
        brain.life_threshold = brain.life_threshold.clamp(0.0, 1.0);
    }
}


////////////////////////////////////////////////////////////////////////////////



fn verify_model2(model_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the model from JSON string
    let model: WormModel = serde_json::from_str(model_json)?;
    println!("Loaded model with parameters:");
    println!("  Food attraction: {:.2}", model.brain.food_attraction);
    println!("  Speed factor: {:.2}", model.brain.speed_factor);
    println!("  Life threshold: {:.2}", model.brain.life_threshold);

    // Create test environment with randomly placed foods
    let mut rng = rand::thread_rng();
    let mut foods: Vec<Food> = (0..5)  // Start with 5 pieces of food
        .map(|i| Food {
            position: (
                rng.gen_range(0..=100),
                rng.gen_range(0..=100)
            ),
            value: rng.gen_range(2..=10)
        })
        .collect();

    // Initialize worm at center
    let mut worm = Worm {
        position: (50.0, 50.0),
        life: 1.0,
        food_consumed: 0,
    };

    println!("\nInitial setup:");
    println!("Worm starting position: ({:.1}, {:.1})", worm.position.0, worm.position.1);
    println!("Food positions:");
    for (i, food) in foods.iter().enumerate() {
        println!("  Food {}: pos ({}, {}), value {}",
                 i + 1,
                 food.position.0,
                 food.position.1,
                 food.value);
    }

    // Run simulation
    let max_steps = 1000;

    println!("\nStarting simulation...");
    for step in 0..max_steps {
        if worm.life <= 0.0 {
            println!("\nWorm died at step {}", step);
            break;
        }

        if foods.is_empty() {
            println!("\nAll food consumed at step {}", step);
            break;
        }

        // Calculate movement direction using brain
        let direction = model.brain.calculate_direction(worm.position, &foods);

        // Adjust speed based on life threshold
        let speed = model.brain.speed_factor *
            if worm.life < model.brain.life_threshold { 1.5 } else { 1.0 };

        // Move worm
        let new_x = worm.position.0 + direction.cos() * speed;
        let new_y = worm.position.1 + direction.sin() * speed;

        // Apply boundaries
        worm.position.0 = new_x.max(0.0).min(100.0);
        worm.position.1 = new_y.max(0.0).min(100.0);

        // Check for food consumption
        let initial_food_count = foods.len();
        foods.retain(|food| {
            let distance = ((worm.position.0 - food.position.0 as f64).powi(2)
                + (worm.position.1 - food.position.1 as f64).powi(2)).sqrt();

            if distance < 2.0 {
                worm.food_consumed += food.value;
                worm.life = (worm.life + 0.2).min(1.0);
                false  // Remove the food
            } else {
                true   // Keep the food
            }
        });

        // If food was consumed this step
        if foods.len() < initial_food_count {
            println!("\nStep {}: Food consumed!", step);
            println!("  Position: ({:.1}, {:.1})", worm.position.0, worm.position.1);
            println!("  Life: {:.2}", worm.life);
            println!("  Total food consumed: {}", worm.food_consumed);
            println!("  Remaining food: {}", foods.len());
        }

        // Decrease life based on movement
        worm.life -= 0.001 * speed;

        // Log status every 100 steps
        if step % 100 == 0 {
            println!("\nStep {} status:", step);
            println!("  Position: ({:.1}, {:.1})", worm.position.0, worm.position.1);
            println!("  Direction: {:.1}°", direction.to_degrees());
            println!("  Speed: {:.2}", speed);
            println!("  Life: {:.2}", worm.life);
            println!("  Food consumed: {}", worm.food_consumed);
        }
    }

    // Print final statistics
    println!("\nSimulation complete!");
    println!("Final statistics:");
    println!("  Total food consumed: {}", worm.food_consumed);
    println!("  Final position: ({:.1}, {:.1})", worm.position.0, worm.position.1);
    println!("  Final life: {:.2}", worm.life);
    println!("  Remaining food: {}", foods.len());

    Ok(())
}

#[test]
fn verify() {
    let model_json = r#"{"brain":{"food_attraction":0.7620645569679378,"speed_factor":1.8947409180791541,"life_threshold":0.31420650864268596},"fitness":8}"#;
    if let Err(e) = verify_model2(model_json) {
        eprintln!("Error verifying model: {}", e);
    }
}