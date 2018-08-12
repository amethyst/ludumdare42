use amethyst::ecs::{Component, DenseVecStorage};
use data::Direction;

#[derive(Deserialize, PartialEq, Clone)]
pub struct BeatPoint {
    pub direction: Direction,
    pub time: f64,
}

impl Component for BeatPoint {
    type Storage = DenseVecStorage<Self>;
}
