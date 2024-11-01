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