#[derive(PartialEq, Eq)]
pub enum AnimationState {
    Running,
    Jumping,
    Falling,
    None,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::None
    }
}