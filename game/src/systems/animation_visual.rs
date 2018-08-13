use amethyst::core::{Time, Transform};
use amethyst::ecs::*;
use amethyst::renderer::{SpriteRender, SpriteSheetSet};

use floating_duration::{TimeAsFloat, TimeFormat};

use data::{AnimationState, Player};

const RUNNING_ANIMATION_SLICE_TIME: f64 = 0.1;

pub struct AnimationVisual {
    current_anim: AnimationState,
    time: f64,
}

impl AnimationVisual {
    pub fn new() -> Self {
        Self {
            current_anim: AnimationState::None,
            time: 0.0,
        }
    }
}

impl<'a> System<'a> for AnimationVisual {
    type SystemData = (
        Read<'a, AnimationState>,
        Read<'a, SpriteSheetSet>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, SpriteRender>,
        Read<'a, Time>,
    );

    fn run(&mut self, (state, set, player, mut sprites, time): Self::SystemData) {
        match *state {
            AnimationState::Running => {
                if let Some((_, sprite)) = (&player, &mut sprites).join().next() {
                    sprite.sprite_sheet = set.handle(101).expect("Running spritesheet not found");
                    if self.current_anim != AnimationState::Running {
                        self.current_anim = AnimationState::Running;
                        self.time = 0.0;
                    } else {
                        self.time = (self.time + time.delta_time().as_fractional_secs())
                            % (RUNNING_ANIMATION_SLICE_TIME * 4.0);
                    }

                    if self.time < RUNNING_ANIMATION_SLICE_TIME {
                        sprite.sprite_number = 0;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 2.0 {
                        sprite.sprite_number = 1;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 3.0 {
                        sprite.sprite_number = 2;
                    } else {
                        sprite.sprite_number = 3;
                    }
                }
            }
            _ => {}
        }
    }
}
