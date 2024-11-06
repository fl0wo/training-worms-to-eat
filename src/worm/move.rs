use crate::math::add_vec2;
use crate::worm::Worm;

pub fn move_worms(worms: &mut Vec<Worm>) {
    for worm in worms.iter_mut() {
        move_worm(worm);
    }
}

fn move_worm(worm: &mut Worm) {
    worm.prev_pos = worm.pos;
    worm.pos = add_vec2(worm.pos, worm.dir, worm.speed * 10.0);
    worm.rotation = worm.dir.y.atan2(worm.dir.x);
}
