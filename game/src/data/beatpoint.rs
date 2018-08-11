use data::Direction;

#[derive(Deserialize)]
pub struct BeatPoint {
    pub direction: Direction,
    pub time: f64,
}
