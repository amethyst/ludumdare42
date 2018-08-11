use data::HitResult;

pub struct GameplayResult {
    pub results: Vec<(f64, HitResult)>,
}

impl Default for GameplayResult {
    fn default() -> Self {
        GameplayResult { results: vec![] }
    }
}
