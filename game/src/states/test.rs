use amethyst::{GameData, State, StateData, Trans};
use amethyst::renderer::Event;

/// The TestState `State`.
#[derive(Default)]
pub struct TestState;

impl TestState {
    /// Creates a new TestState.
    pub fn new() -> Self {
        TestState {}
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for TestState {
    fn on_start(&mut self, mut data: StateData<GameData>) {}

    fn on_stop(&mut self, mut data: StateData<GameData>) {}

    fn on_pause(&mut self, mut data: StateData<GameData>) {}

    fn on_resume(&mut self, mut data: StateData<GameData>) {}

    fn handle_event(
        &mut self,
        mut data: StateData<GameData>,
        _event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        Trans::None
    }

    fn fixed_update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        Trans::None
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}
