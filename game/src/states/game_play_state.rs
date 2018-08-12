use amethyst::core::Transform;
use amethyst::ecs::prelude::*;
use amethyst::input::{get_key, is_close_requested};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::{GameData, State, StateData, Trans};

use CameraFollowPlayerSystem;
use GameplayInputSystem;
use GameplayResult;
use GameplayStatus;
use Player;
use ScoreState;

/// Where the player is running out of space
#[derive(Default, new)]
pub struct GamePlayState {
    /// State specific dispatcher
    #[new(default)]
    dispatcher: Option<Dispatcher<'static, 'static>>,
    /// Whether or not the game is paused.
    #[new(value = "false")]
    paused: bool,
    /// All entities in game.
    #[new(default)]
    entities: Vec<Entity>,
}

impl GamePlayState {
    fn initialize_dispatcher(&mut self, world: &mut World) {
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(GameplayInputSystem::new(), "gameplay_input_system", &[]);
        dispatcher_builder.add(PlayerMovementSystem::new(), "player_movement", &[]);
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

    fn initialize_entities(&mut self, world: &mut World) {
        let player = world
            .create_entity()
            .with(Player::default())
            .with(Transform::default())
            .build();

        self.entities.push(player);

        // Load scene prefab
        let mut beatmap_name = world.write_resource::<BeatMap>().name.clone();
        let scene_path = world.read_resource::<AssetLoader>().resolve_path(&format!("maps/{}/map.ron", beatmap_name)).expect(&format!("Please ensure map.ron::name == name of the folder containing map.ron for map {}",beatmap_name));



        // Map beatpoint visual components to beatmap beatpoints
        //world.write_resource::<BeatMap>().beat_points =
        let beatpoints = (&world.read_storage::<BeatPoint>(),).join().iter().cloned().
    }

    fn terminate_entities(&mut self, world: &mut World) {
        self.entities.drain(..).for_each(|entity| {
            world
                .delete_entity(entity)
                .expect("Failed to delete game entity.")
        });
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting GamePlayState");
        self.initialize_dispatcher(&mut data.world);
        self.initialize_entities(&mut data.world);

        // TODO: create beat points from BeatMap
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_entities(&mut data.world);
        self.terminate_dispatcher();
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) {
            return Trans::Quit;
        }

        match get_key(&event) {
            Some((VirtualKeyCode::Escape, ElementState::Pressed)) => Trans::Pop,
            Some((VirtualKeyCode::Space, ElementState::Pressed)) => {
                self.paused = !self.paused;
                if self.paused {
                    info!("Game is paused.");
                } else {
                    info!("Game is running.");
                }

                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);

        // TODO: Probably render something on screen to say "the game is paused"
        // Should we also add an entity with a `Paused` component that indicates the paused state?
        if !self.paused {
            self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);
        }

        let gameplay_result = &data.world.read_resource::<GameplayResult>();
        match gameplay_result.status {
            GameplayStatus::Failed | GameplayStatus::Completed => {
                Trans::Switch(Box::new(ScoreState::new()))
            }
            _ => Trans::None,
        }
    }
}
