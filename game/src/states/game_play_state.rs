use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::{GameData, State, StateData, Trans};

use ScoreState;

/// Where the player is running out of space
#[derive(Default)]
pub struct GamePlayState;

impl GamePlayState {
    /// Creates a new GamePlayState.
    pub fn new() -> Self {
        GamePlayState
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting GamePlayState");
        // TODO: load maps / songs
    }

    fn handle_event(
        &mut self,
        mut data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) {
            return Trans::Quit;
        }

        match get_key(&event) {
            Some((VirtualKeyCode::Escape, ElementState::Pressed)) => Trans::Pop,
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);

        // TODO: check game progress, if song is finished / player is too far back, then switch

        Trans::Switch(Box::new(ScoreState::new()))
    }
}
