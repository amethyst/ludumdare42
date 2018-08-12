use amethyst::core::{Time, Transform};
use amethyst::ecs::*;

use data::FallingObject;

pub struct MakeObjectsFall;

impl<'a> System<'a> for MakeObjectsFall {
    type SystemData = (
        WriteStorage<'a, FallingObject>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut falling, mut trans, time): Self::SystemData) {
        for (f, t) in (&mut falling, &mut trans).join() {
            if f.falling {
                if t.translation.y > f.ground_level {
                    t.translation.y =
                        (t.translation.y - time.delta_seconds() * f.velocity).max(f.ground_level);
                } else {
                    f.falling = false;
                }
            }
        }
    }
}
