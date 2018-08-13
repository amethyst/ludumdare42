use amethyst::core::{Time, Transform};
use amethyst::ecs::*;
use amethyst::renderer::{SpriteRender, SpriteSheetSet};

use floating_duration::{TimeAsFloat, TimeFormat};

use data::*;

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
        Read<'a, AnimationStateRes>,
        Read<'a, SpriteSheetSet>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, SpriteRender>,
        Read<'a, Time>,
    );

    fn run(&mut self, (state, set, player, mut sprites, time): Self::SystemData) {
        match state.state {
            AnimationState::Running => {
                if let Some((_, sprite)) = (&player, &mut sprites).join().next() {
                    if self.current_anim != AnimationState::Running {
                        self.current_anim = AnimationState::Running;
                        self.time = 0.0;
                        sprite.sprite_sheet =
                            set.handle(101).expect("Running spritesheet not found");
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
            AnimationState::Jumping => {
                if let Some((_, sprite)) = (&player, &mut sprites).join().next() {
                    if self.current_anim != AnimationState::Jumping {
                        sprite.sprite_sheet =
                            set.handle(100).expect("Jumping spritesheet not found");
                        self.current_anim = AnimationState::Jumping;
                        self.time = 0.0;
                    } else {
                        self.time = (self.time + time.delta_time().as_fractional_secs())
                            % (RUNNING_ANIMATION_SLICE_TIME * 6.0);
                    }

                    if self.time < RUNNING_ANIMATION_SLICE_TIME {
                        sprite.sprite_number = 0;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 2.0 {
                        sprite.sprite_number = 1;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 3.0 {
                        sprite.sprite_number = 2;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 4.0 {
                        sprite.sprite_number = 3;
                    } else {
                        sprite.sprite_number = 4;
                    }
                }
            }
            AnimationState::Falling => {
                if let Some((_, sprite)) = (&player, &mut sprites).join().next() {
                    if self.current_anim != AnimationState::Falling {
                        sprite.sprite_sheet =
                            set.handle(102).expect("Falling spritesheet not found");
                        self.current_anim = AnimationState::Falling;
                        self.time = 0.0;
                    } else {
                        self.time = (self.time + time.delta_time().as_fractional_secs())
                            % (RUNNING_ANIMATION_SLICE_TIME * 18.0);
                    }

                    if self.time < RUNNING_ANIMATION_SLICE_TIME {
                        sprite.sprite_number = 0;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 2.0 {
                        sprite.sprite_number = 1;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 3.0 {
                        sprite.sprite_number = 2;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 4.0 {
                        sprite.sprite_number = 3;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 5.0 {
                        sprite.sprite_number = 4;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 6.0 {
                        sprite.sprite_number = 5;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 7.0 {
                        sprite.sprite_number = 6;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 8.0 {
                        sprite.sprite_number = 7;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 9.0 {
                        sprite.sprite_number = 8;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 10.0 {
                        sprite.sprite_number = 9;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 11.0 {
                        sprite.sprite_number = 10;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 12.0 {
                        sprite.sprite_number = 11;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 13.0 {
                        sprite.sprite_number = 12;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 14.0 {
                        sprite.sprite_number = 13;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 15.0 {
                        sprite.sprite_number = 14;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 16.0 {
                        sprite.sprite_number = 15;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 17.0 {
                        sprite.sprite_number = 16;
                    // TODO: kms
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 18.0 {
                        sprite.sprite_number = 17;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 19.0 {
                        sprite.sprite_number = 18;
                    } else if self.time < RUNNING_ANIMATION_SLICE_TIME * 20.0 {
                        sprite.sprite_number = 19;
                    } else {
                        sprite.sprite_number = 20;
                    }
                }
            }
            _ => {}
        }
    }
}
