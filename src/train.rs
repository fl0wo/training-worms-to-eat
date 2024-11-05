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
    position: (f64, f64), // Changed to f64 for precision
    life: f64,            // Life points, max 1.0
    food_consumed: i32,
}

#[derive(Debug, Clone)]
struct GivenFoods {
    list: Vec<Food>,
}

impl From<Vec<Food>> for GivenFoods {
    fn from(foods: Vec<Food>) -> Self {
        GivenFoods { list: foods }
    }
}

#[derive(Debug)]
struct Problem {
    given_foods: GivenFoods,
}

type Selection = Vec<f64>; // Change to store angles (in radians)

trait AsPhenotype {
    fn as_worm(&self, given_foods: &GivenFoods) -> Worm;
}

impl AsPhenotype for Selection {
    fn as_worm(&self, given_foods: &GivenFoods) -> Worm {
        let mut worm = Worm {
            position: (0.0, 0.0), // Start at origin
            life: 1.0,            // Full life at the start
            food_consumed: 0,
        };

        let mut rng = rand::thread_rng();
        let speed = rng.gen_range(1.0..=10.0); // Random speed between 1 and 10

        for decision in self.iter() {
            // Use the angle directly to move
            let direction = *decision; // This is now in radians

            // Update worm's position based on speed and direction
            worm.position.0 += direction.cos() * speed;
            worm.position.1 += direction.sin() * speed;

            // Check if the worm is at a food position and consume food
            for food in &given_foods.list {
                if ((worm.position.0 - food.position.0 as f64).powi(2)
                    + (worm.position.1 - food.position.1 as f64).powi(2))
                    .sqrt() < 1.0 // Threshold for consuming food
                {
                    worm.food_consumed += food.value;
                    worm.life = (worm.life + 0.1).min(1.0); // Regain some life on food consumption
                }
            }

            // Worm loses life based on speed
            worm.life -= 0.01 * (speed / 10.0); // Loss proportionate to speed
            if worm.life < 0.0 {
                worm.life = 0.0; // Ensure life doesn't go negative
            }
        }

        worm
    }
}

impl Problem {
    pub fn new(given_foods: GivenFoods) -> Self {
        Self { given_foods }
    }
}

impl<'a> FitnessFunction<Selection, i32> for &'a Problem {
    fn fitness_of(&self, selection: &Selection) -> i32 {
        let worm = selection.as_worm(&self.given_foods);
        let mut fitness = worm.food_consumed;

        // Add distance penalty based on food positions
        let min_distance = self.given_foods.list.iter()
            .map(|food| {
                ((worm.position.0 - food.position.0 as f64).powi(2)
                    + (worm.position.1 - food.position.1 as f64).powi(2))
                    .sqrt()
            })
            .fold(None, |acc: Option<f64>, distance| {
                match acc {
                    Some(min) => Some(min.min(distance)),
                    None => Some(distance),
                }
            });


        fitness -= min_distance.unwrap() as i32; // Penalize based on distance to the closest food

        // Ensure fitness is not negative
        fitness = fitness.max(0);
        fitness
    }

    fn average(&self, values: &[i32]) -> i32 {
        (values.iter().sum::<i32>() as f32 / values.len() as f32 + 0.5).floor() as i32
    }

    fn highest_possible_fitness(&self) -> i32 {
        self.given_foods.list.iter().map(|f| f.value).sum()
    }

    fn lowest_possible_fitness(&self) -> i32 {
        0
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WormModel {
    genome: Selection,
    fitness: i32,
}

fn save_model(model: &WormModel, filename: &str) -> std::io::Result<()> {
    let serialized = serde_json::to_string(model)?;
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

#[test]
fn train() {
    let given_foods: GivenFoods = vec![
        Food { position: (rand_int(0, 100), rand_int(0, 100)), value: rand_int(2, 10) },
        Food { position: (rand_int(0, 100), rand_int(0, 100)), value: rand_int(2, 10) },
        Food { position: (rand_int(0, 100), rand_int(0, 100)), value: rand_int(2, 10) },
        Food { position: (rand_int(0, 100), rand_int(0, 100)), value: rand_int(2, 10) },
    ].into();

    let problem = Problem::new(given_foods);

    // Create initial population without range restrictions
    let initial_population: Population<Selection> = build_population()
        .with_genome_builder(ValueEncodedGenomeBuilder::new(100, 0.0, 2.0 * std::f64::consts::PI)) // Allowing angles between 0 and 2π
        .of_size(100)
        .uniform_at_random();

    let mut worm_sim = simulate(
        genetic_algorithm()
            .with_evaluation(&problem)
            .with_selection(MaximizeSelector::new(0.85, 12))
            .with_crossover(SinglePointCrossBreeder::new())
            .with_mutation(RandomValueMutator::new(0.3f64, 0.1, 1f64)) // Increased mutation rate
            .with_reinsertion(ElitistReinserter::new(&problem, false, 0.85))
            .with_initial_population(initial_population)
            .build()
    )
        .until(GenerationLimit::new(1000))
        .build();

    'sim: loop {
        let result = worm_sim.step();

        match result {
            Ok(SimResult::Intermediate(step)) => {
                let evaluated_population = step.result.evaluated_population;
                let best_solution = step.result.best_solution;
                println!(
                    "Step: generation: {}, average_fitness: {}, \
                     best fitness: {}, duration: {}, processing_time: {}",
                    step.iteration,
                    evaluated_population.average_fitness(),
                    best_solution.solution.fitness,
                    step.duration.fmt(),
                    step.processing_time.fmt(),
                );
            },
            Ok(SimResult::Final(step, processing_time, duration, stop_reason)) => {
                let best_solution = step.result.best_solution;
                println!("{}", stop_reason);
                println!(
                    "Final result after {}: generation: {}, \
                     best solution with fitness {} found in generation {}, processing_time: {}",
                    duration.fmt(),
                    step.iteration,
                    best_solution.solution.fitness,
                    best_solution.generation,
                    processing_time.fmt(),
                );

                // Create the WormModel instance
                let worm_model = WormModel {
                    genome: best_solution.solution.genome.clone(),
                    fitness: best_solution.solution.fitness,
                };

                // Save the model to a file
                if let Err(e) = save_model(&worm_model, "worm_model.json") {
                    eprintln!("Failed to save model: {}", e);
                } else {
                    println!("Model saved successfully!");
                }

                break 'sim;
            },
            Err(error) => {
                println!("{}", error);
                break 'sim;
            },
        }
    }
}


fn load_model(filename: &str) -> std::io::Result<WormModel> {
    let file = File::open(filename)?;
    let model: WormModel = serde_json::from_reader(file)?;
    Ok(model)
}

fn move_worm(model: &WormModel, given_foods: &GivenFoods) -> Worm {
    let mut worm = Worm {
        position: (0.0, 0.0), // Start at the origin
        life: 1.0,            // Full life at the start
        food_consumed: 0,
    };

    let mut rng = rand::thread_rng();
    let speed = rng.gen_range(1.0..=10.0); // Random speed between 1 and 10

    for decision in &model.genome {
        // Use the angle directly to move
        let direction = *decision; // This is now in radians

        // Update worm's position based on speed and direction
        worm.position.0 += direction.cos() * speed;
        worm.position.1 += direction.sin() * speed;

        // Check if the worm is at a food position and consume food
        for food in &given_foods.list {
            if ((worm.position.0 - food.position.0 as f64).powi(2)
                + (worm.position.1 - food.position.1 as f64).powi(2))
                .sqrt() < 1.0 // Threshold for consuming food
            {
                worm.food_consumed += food.value;
            }
        }

        // Worm loses life based on speed
        worm.life -= 0.01 * (speed / 10.0); // Loss proportionate to speed
        if worm.life < 0.0 {
            worm.life = 0.0; // Ensure life doesn't go negative
        }
    }

    worm
}
fn verify_model(filename: &str) -> Result<(), String> {
    // Load the model from the file
    let worm_model = load_model(filename).map_err(|e| e.to_string())?;

    // Generate random food positions
    let mut rng = rand::thread_rng();
    let given_foods: GivenFoods = (0..10) // Place 10 pieces of food
        .map(|_| Food {
            position: (rng.gen_range(0..=100), rng.gen_range(0..=100)), // Random positions
            value: rng.gen_range(2..=10), // Random values
        })
        .collect::<Vec<_>>()
        .into();

    // Create a worm from the model's genome
    let mut worm = Worm {
        position: (0.0, 0.0), // Start at the origin
        life: 1.0,            // Full life at the start
        food_consumed: 0,
    };

    let speed = rng.gen_range(1.0..=10.0); // Random speed between 1 and 10
    let iterations = 1000; // Define the number of movements

    for decision in &worm_model.genome {
        // Move the worm for the defined number of iterations
        for _ in 0..iterations {
            // Use the angle directly to move
            let direction = *decision; // This is now in radians
            let direction_degrees = direction.to_degrees(); // Convert to degrees

            // Check the distance to food and determine if it's in the right direction
            let mut is_towards_food = false;
            for food in &given_foods.list {
                let distance = ((worm.position.0 - food.position.0 as f64).powi(2)
                    + (worm.position.1 - food.position.1 as f64).powi(2))
                    .sqrt();
                if distance < food.value as f64 {
                    // Log if the worm is moving towards food
                    is_towards_food = true;
                    break;
                }
            }

            // Log the decision made by the worm
            let direction_info = if is_towards_food {
                format!("Direction: {:.2}° (Towards food)", direction_degrees)
            } else {
                format!("Direction: {:.2}° (Not towards food)", direction_degrees)
            };
            println!("{}", direction_info);

            // Update worm's position based on speed and direction
            worm.position.0 += direction.cos() * speed;
            worm.position.1 += direction.sin() * speed;

            // Check if the worm is at a food position and consume food
            for food in &given_foods.list {
                if ((worm.position.0 - food.position.0 as f64).powi(2)
                    + (worm.position.1 - food.position.1 as f64).powi(2))
                    .sqrt() < food.value as f64 // Threshold for consuming food
                {
                    worm.food_consumed += food.value;
                }
            }

            // Worm loses life based on speed
            worm.life -= 0.01 * (speed / 10.0); // Loss proportionate to speed
            if worm.life <= 0.0 {
                worm.life = 0.0; // Ensure life doesn't go negative
                break; // Stop the loop if the worm is dead
            }
        }
    }

    // Log the results
    println!("Final Worm Position: {:?}", worm.position);
    println!("Life: {}", worm.life);
    println!("Food Consumed: {}", worm.food_consumed);

    Ok(())
}





#[test]
fn verify() {
    if let Err(e) = verify_model("worm_model.json") {
        eprintln!("Failed to verify model: {}", e);
    } else {
        println!("Model verified successfully!");
    }
}