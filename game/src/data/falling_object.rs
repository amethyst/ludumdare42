use amethyst::ecs::*;

pub struct FallingObject {
    pub ground_level: f32,
    pub velocity: f32,
    pub falling: bool,
}

impl FallingObject {
    pub fn new(ground_level: f32, velocity: f32) -> Self {
        Self {
            ground_level,
            velocity,
            falling: false,
        }
    }
}

impl Component for FallingObject {
    type Storage = VecStorage<FallingObject>;
}