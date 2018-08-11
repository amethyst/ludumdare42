#[derive(Clone)]
pub enum HitResult {
    Hit,
    MissEarly,
    MissLate,
    MissKey,
}
