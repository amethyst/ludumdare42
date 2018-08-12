use amethyst::core::{Time, Transform};
use amethyst::ecs::*;
use amethyst::core::cgmath::Deg;

use data::RotatingObject;

pub struct MakeObjectsRotate;

impl<'a> System<'a> for MakeObjectsRotate {
    type SystemData = (
        ReadStorage<'a, RotatingObject>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
    );

    fn run(&mut self, (rotating, mut trans, time): Self::SystemData) {
        for (r, t) in (&rotating, &mut trans).join() {
            t.roll_local(Deg(r.velocity * time.delta_seconds()));
        }
    }
}
