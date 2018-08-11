use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Debug, new)]
pub struct BeatmapButton {
    pub beatmap: String,
}

impl Component for BeatmapButton {
    type Storage = DenseVecStorage<Self>;
}
