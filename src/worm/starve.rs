use raylib::check_collision_circles;
use crate::food::{Food, ray_of_food};
use crate::worm::Worm;

/**
Based on how fast the worms are moving, they will lose energy.
 */
pub fn starve_worms(worms: &mut Vec<Worm>) {

    let factor = 0.001;

    for worm in worms.iter_mut() {
        // TODO: this should not be a linear punishment, it should be exponential (sprinting consumes much more energy)
        worm.life -= factor * worm.speed;
    }

    worms.retain(|worm| worm.life > 0.0);
}

pub fn feed_worms(worms: &mut Vec<Worm>, food: &mut Vec<Food>) {
    let max_eat:f32 = 1.0;

    // For each worm, check if it is colliding with any food
    for worm in worms.iter_mut() {
        for f in food.iter_mut() {
            if f.amount <= 0.0 {
                continue;
            }

            if check_collision_circles(worm.pos, worm.ray, f.pos, ray_of_food(f)) {
                worm.life = (worm.life + 2.0 * max_eat).min(1.0);
                f.amount -= f.amount.min(max_eat) / 100f32;
            }
        }
    }
}