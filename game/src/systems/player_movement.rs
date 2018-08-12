use amethyst::ecs::{Join, Read, ReadExpect, ReadStorage, Resources, System, SystemData, Write,
                    WriteExpect, WriteStorage};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::input::get_key;
use amethyst::core::{Time, Transform};
use amethyst::shrev::{EventChannel, ReaderId};

use data::*;

pub struct PlayerMovementSystem {
    last_beatpoint: Option<BeatPoint>,
    /// TODO: Remove and use the timing logic instead
    /// Will break with multiple players (lol)
    in_transit: bool,
}

impl PlayerMovementSystem {
    pub fn new() -> Self {
        PlayerMovementSystem {
            last_beatpoint: None,
            in_transit: false,
        }
    }
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        ReadExpect<'a, BeatMap>,
        Read<'a, GameplayResult>,
    );

    fn run(&mut self, (players, mut transforms, time, beatmap, gameplay_result): Self::SystemData) {
        let time_to_node_mult = 0.1;
        let rel_time = time.absolute_time_seconds() - beatmap.runtime_start;

        if let Some(front) = beatmap.beat_points.front() {
            if self.last_beatpoint.is_none() {
                self.last_beatpoint = Some(front.clone());
            } else if self.last_beatpoint.as_ref().unwrap() != front && !self.in_transit {
                self.last_beatpoint = Some(front.clone());
            }

            /*
            if done transition, last node = current
            if in transition: transition // breaks on 2+ beatpoint in a single frame (but input too anyway)


            transition = move player pos lerp  lastnode->current  * (deltatime / time_to_node_mult)
            */

            // TODO: finish it, see previous comment for planned algorithm

            /*if self.last_beatpoint !=
            let last_beatpoint = ???;*/

            //if self.last_beatpoint.is_none() ||
        }
    }
}
