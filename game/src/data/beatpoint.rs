use data::Direction;

#[derive(Deserialize, PartialEq)]
pub struct BeatPoint {
    pub direction: Direction,
    pub time: f64,
}
