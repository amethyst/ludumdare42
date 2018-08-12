#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
