use amethyst::core::cgmath::Vector3;
use amethyst::core::{Time, Transform};
use amethyst::ecs::{
    Join, Read, ReadExpect, ReadStorage, Resources, System, SystemData, Write, WriteExpect,
    WriteStorage,
};
use amethyst::input::get_key;
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::shrev::{EventChannel, ReaderId};

use data::*;
use std::collections::VecDeque;

pub struct PlayerMovementSystem {
    last_beatpoint: Option<(Vector3<f32>, f64)>,
    beat_points: Option<VecDeque<(Vector3<f32>, f64)>>,
}

impl PlayerMovementSystem {
    pub fn new() -> Self {
        PlayerMovementSystem {
            last_beatpoint: None,
            beat_points: None,
        }
    }
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        Read<'a, Time>,
        ReadExpect<'a, BeatMap>,
        ReadStorage<'a, BeatPoint>,
        Write<'a, GameplayResult>,
    );

    fn run(
        &mut self,
        (players, mut transforms, time, beatmap, beatpoints, mut gameplay_result): Self::SystemData,
    ) {
        let time_to_node_mult = 0.5;
        let rel_time = time.absolute_time_seconds() - beatmap.runtime_start;

        if self.beat_points.is_none() {
            let mut v = Vec::<(Vector3<f32>, f64)>::new();
            for (transform, beatpoint) in (&transforms, &beatpoints).join() {
                v.push((transform.translation.clone(), beatpoint.time));
            }
            if !v.is_empty() {
                v.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                self.beat_points = Some(v.into());
            }
        } else {
            // FIXME: Skips the first frame

            while !self.beat_points.as_ref().unwrap().is_empty()
                && self.beat_points.as_ref().unwrap().front().map(|t| t.1)
                    != beatmap.beat_points.front().map(|b| b.time)
            {
                self.beat_points.as_mut().unwrap().pop_front();
            }

            for (mut transform, _) in (&mut transforms, &players).join() {
                if self.last_beatpoint.is_none() {
                    self.last_beatpoint = Some((transform.translation.clone(), 0.0));
                }

                if self.beat_points.as_ref().unwrap().is_empty() {
                    return;
                }

                // if in transition
                // if last beatpoint != current, start transitionning
                if self.last_beatpoint.as_ref() != self.beat_points.as_ref().unwrap().front() {
                    let trans_time_start = self.last_beatpoint.as_ref().unwrap().1;
                    let trans_time_stop = self.beat_points.as_ref().unwrap().front().unwrap().1;

                    let trans_duration = (trans_time_stop - trans_time_start) * time_to_node_mult;

                    let dir = self.beat_points.as_ref().unwrap().front().unwrap().0
                        - self.last_beatpoint.as_ref().unwrap().0;
                    let new_pos = self.last_beatpoint.as_ref().unwrap().0
                        + dir * ((rel_time - trans_time_start) / trans_duration) as f32;

                    transform.translation = new_pos;

                    // if arrived, last beatpoint = current
                    if rel_time >= trans_time_start + trans_duration {
                        self.last_beatpoint =
                            Some(self.beat_points.as_ref().unwrap().front().unwrap().clone());
                        transform.translation = self
                            .beat_points
                            .as_ref()
                            .unwrap()
                            .front()
                            .unwrap()
                            .0
                            .clone();

                        // if this is the last point, the game is done.
                        if self.beat_points.as_ref().unwrap().len() <= 1 {
                            gameplay_result.status = GameplayStatus::Completed;
                        }
                    }
                }
            }
        }
    }
}
