use amethyst::ecs::{Component, VecStorage};

pub struct Player {
    pub health: i32,
    pub velocity: f32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

impl Default for Player {
    fn default() -> Self {
        Player { health: 10, velocity: 1.0 }
    }
}
