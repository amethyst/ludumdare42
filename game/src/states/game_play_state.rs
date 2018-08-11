use amethyst::ecs::prelude::*;
use amethyst::input::{get_key, is_close_requested};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::{GameData, State, StateData, Trans};

use CameraFollowPlayerSystem;
use GameplayInputSystem;
use GameplayResult;
use GameplayStatus;
use ScoreState;

/// Where the player is running out of space
#[derive(Default, new)]
pub struct GamePlayState {
    /// State specific dispatcher
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
}

impl GamePlayState {
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        // FIXME: jojolepro is this correct?
        dispatcher_builder.add(GameplayInputSystem::new(), "gameplay_input_system", &[]);
        dispatcher_builder.add(
            CameraFollowPlayerSystem,
            "camera_follow_player_system",
            &["gameplay_input_system"],
        );

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting GamePlayState");
        self.initialize_dispatcher(&mut data.world);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.terminate_dispatcher();
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
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
        self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);

        let gameplay_result = &data.world.read_resource::<GameplayResult>();
        match gameplay_result.status {
            GameplayStatus::Failed | GameplayStatus::Completed => {
                Trans::Switch(Box::new(ScoreState::new()))
            }
            _ => Trans::None,
        }
    }
}
