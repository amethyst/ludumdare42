use amethyst::ecs::{Component,VecStorage};


pub struct Player {
    pub health: i32,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 10,
        }
    }
}