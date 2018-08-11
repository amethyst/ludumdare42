use data::*;

pub struct GameplayResult {
    pub results: Vec<(f64, HitResult)>,
    pub status: GameplayStatus,
}

impl Default for GameplayResult {
    fn default() -> Self {
        GameplayResult {
            results: vec![],
            status: GameplayStatus::Running,
        }
    }
}
