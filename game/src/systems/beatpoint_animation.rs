use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, Resources, System, SystemData, Write, WriteExpect, WriteStorage};
use amethyst::core::{Transform, GlobalTransform};
use amethyst::renderer::SpriteRender;
use amethyst::core::timing::Time;

use data::*;

#[derive(Default)]
/// The BeatPointAnimationSystem `System`.
pub struct BeatPointAnimationSystem;

impl<'a> System<'a> for BeatPointAnimationSystem{
    // Don't forgot to add a trailing , in both parenthesis
    type SystemData = (Read<'a, Time>,
        ReadExpect<'a, BeatMap>,
        ReadStorage<'a, BeatPoint>,
        WriteStorage<'a, SpriteRender>,
    );
    
    fn run(&mut self,(time, beatmap, beatpoints, mut sprites): Self::SystemData) {
        let rel_time = time.absolute_time_seconds() - beatmap.runtime_start;
        let stage_length = 0.1;
        let max_stage = 4; // [0,4]
        for (beatpoint, mut sprite) in (&beatpoints, &mut sprites).join() {
            let dir_number = match beatpoint.direction {
                Direction::Up => 0,
                Direction::Down => 1,
                Direction::Right => 2,
                Direction::Left => 3,
            };
            let mut time_left = beatpoint.time - rel_time;
            if time_left < 0.0 {
                time_left = 0.0;
            }
            let mut stage = (time_left / stage_length) as usize;
            if stage > max_stage {
                stage = max_stage;
            }
            sprite.sprite_number = stage + (5 * dir_number);
        }
    }
}
