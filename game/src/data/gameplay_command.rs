/// Tells the GameplayState what to do next
pub enum GameplayCommand {
    BackToMenu,
    Retry,
}

impl Default for GameplayCommand {
    fn default() -> Self {
        GameplayCommand::BackToMenu
    }
}