use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::{GameData, State, StateData, Trans};

use GamePlayState;

/// Where the player chooses which song to play
#[derive(Default)]
pub struct MapSelectionState;

impl MapSelectionState {
    /// Creates a new MapSelectionState.
    pub fn new() -> Self {
        MapSelectionState
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for MapSelectionState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting MapSelectionState");
        // TODO: load maps / songs
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        debug!("Resuming MapSelectionState");
        // TODO: load maps / songs
    }

    fn handle_event(
        &mut self,
        mut data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit;
        }
        match get_key(&event) {
            Some((VirtualKeyCode::Up, ElementState::Pressed)) => {
                // TODO: move to previous map / wrap around
                Trans::None
            }

            Some((VirtualKeyCode::Down, ElementState::Pressed)) => {
                // TODO: move to next map / wrap around
                Trans::None
            }

            Some((VirtualKeyCode::Return, ElementState::Pressed)) => {
                // TODO: insert map selection into `World`
                Trans::Push(Box::new(GamePlayState::new()))
            }

            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}
