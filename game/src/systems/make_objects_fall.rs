use amethyst::core::{Time, Transform};
use amethyst::ecs::*;

use data::{FallingObject, Player};

pub struct MakeObjectsFall;

impl<'a> System<'a> for MakeObjectsFall {
    type SystemData = (
        WriteStorage<'a, FallingObject>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut falling, mut trans, player, time): Self::SystemData) {
        if let Some((player_pos, player_vel)) = (&mut trans, &player)
            .join()
            .next()
            .map(|(t, p)| (t.translation.x, p.velocity))
        {
            for (f, t) in (&mut falling, &mut trans).join() {
                if f.falling {
                    if t.translation.y > f.ground_level {
                        t.translation.y = (t.translation.y - time.delta_seconds() * f.velocity)
                            .max(f.ground_level);
                    }
                } else if player_pos - t.translation.x > f.trigger_time * player_vel {
                    f.falling = true;
                }
            }
        }
    }
}
