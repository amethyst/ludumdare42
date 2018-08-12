#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
