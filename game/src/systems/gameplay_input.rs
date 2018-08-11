use amethyst::ecs::{System,ReadStorage,WriteStorage,Join,Read,Write, Resources, WriteExpect,SystemData};
use amethyst::renderer::{Event,VirtualKeyCode,ElementState};
use amethyst::input::get_key;
use amethyst::core::Time;
use amethyst::shrev::{EventChannel,ReaderId};

use data::*;

pub struct GameplayInputSystem {
    input_reader: Option<ReaderId<Event>>,
}

impl GameplayInputSystem {
    pub fn new() -> Self {
        GameplayInputSystem {
            input_reader: None,
        }
    }
}

impl<'a> System<'a> for GameplayInputSystem {
    type SystemData = (WriteStorage<'a,Player>,Read<'a,EventChannel<Event>>,Read<'a,Time>,WriteExpect<'a, BeatMap>,Write<'a, GameplayResult>);

    fn run(&mut self, (mut players,input,time,mut beatmap,mut gameplay_result): Self::SystemData){
        let offset = 0.2;
        let rel_time = time.absolute_time_seconds() - beatmap.runtime_start;

        let mut early_remove = 0;
        // if too late
        for (i,beatpoint) in beatmap.beat_points.iter().enumerate() {
            // too late :(
            if rel_time > beatpoint.time + offset {
                early_remove = early_remove + 1;
            }
        }
        for i in 0..early_remove{
            let missed = beatmap.beat_points.pop_front().unwrap();
            gameplay_result.results.push((missed.time,HitResult::MissLate));
        }


        // check input
        for ev in input.read(&mut self.input_reader.as_mut().unwrap()) {
            if let Some((key,ElementState::Pressed)) = get_key(&ev) {
                // Only consider arrow keys
                if key == VirtualKeyCode::Left || key == VirtualKeyCode::Right || key == VirtualKeyCode::Up || key == VirtualKeyCode::Down {
                    if let Some(beatpoint) = beatmap.beat_points.pop_front() {

                        // too early
                        if rel_time < beatpoint.time - offset {
                            // ignore

                            //gameplay_results.results.push((missed.time,HitResult::MissEarly));

                            //temporary to keep it
                            beatmap.beat_points.push_front(beatpoint);
                        } else {
                            //if ok

                            let correct_key = match key {
                                VirtualKeyCode::Left => beatpoint.direction == Direction::Left,
                                VirtualKeyCode::Right => beatpoint.direction == Direction::Right,
                                VirtualKeyCode::Up => beatpoint.direction == Direction::Up,
                                VirtualKeyCode::Down => beatpoint.direction == Direction::Down,
                                _ => false, // impossible
                            };

                            // TODO: trigger animations (can be done by checking player dist to first beatpoint)
                            if correct_key {
                                gameplay_result.results.push((beatpoint.time, HitResult::Hit));
                            } else {
                                gameplay_result.results.push((beatpoint.time, HitResult::MissKey));
                            }
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, mut res: &mut Resources) {
        Self::SystemData::setup(&mut res);
        self.input_reader = Some(res.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}