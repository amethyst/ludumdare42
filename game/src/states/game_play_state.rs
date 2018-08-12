use amethyst::assets::*;
use amethyst::core::cgmath::{Matrix4, Ortho, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::prelude::*;
use amethyst::input::{get_key, is_close_requested};
use amethyst::renderer::{
    Camera, ElementState, Event, Projection, ScreenDimensions, VirtualKeyCode,
};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::*;

use std::collections::VecDeque;

use data::*;
use systems::PlayerMovementSystem;
use utils::prefabs::SpriteScenePrefab;
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
    /// Camera entity
    #[new(default)]
    camera: Option<Entity>,
    /// Map has been fully loaded
    #[new(value = "false")]
    loaded: bool,
    /// The progress counter of the scene
    #[new(value = "None")]
    progress_counter: Option<ProgressCounter>,
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

        // TODO: remove this. ew
        world.register::<BeatPoint>();

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(&mut world.res);
        self.dispatcher = Some(dispatcher);
    }

    fn terminate_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn initialize_entities(&mut self, world: &mut World) {
        /*let player = world
            .create_entity()
            .with(Player::default())
            .with(Transform::default())
            .build();

        self.entities.push(player);*/

        // Find prefab file to load
        let mut beatmap_name = world.write_resource::<BeatMap>().name.clone();
        let scene_path = world
            .read_resource::<AssetLoader>()
            .resolve_path(&format!("maps/{}/scene.ron", beatmap_name))
            .expect(&format!(
                "Please ensure map.ron::name == name of the folder containing map.ron for map {}",
                beatmap_name
            ));

        // Load the map!
        let mut progress_counter = ProgressCounter::default();
        let prefab_handle = world.exec(|loader: PrefabLoader<SpriteScenePrefab>| {
            // might fail with abs path??
            loader.load(scene_path, RonFormat, (), &mut progress_counter)
        });
        world.create_entity().with(prefab_handle).build();

        self.progress_counter = Some(progress_counter);
    }

    fn terminate_entities(&mut self, world: &mut World) {
        self.entities.drain(..).for_each(|entity| {
            world
                .delete_entity(entity)
                .expect("Failed to delete game entity.")
        });
    }

    /// Initializes a camera to view the game.
    fn initialize_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // The Z coordinate of the camera is how far along it should be before it faces the
        // entities. Anything greater than this will be culled.
        let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, 100.0));
        let global_transform = GlobalTransform(translation);

        let camera = world
            .create_entity()
            .with(Camera::from(Projection::Orthographic(Ortho {
                left: 0.0,
                right: width,
                top: height,
                bottom: 0.0,
                near: 0.0,
                far: 20000.,
            })))
            .with(global_transform)
            .build();

        self.camera = Some(camera);
    }

    /// Terminates the camera.
    fn terminate_camera(&mut self, world: &mut World) {
        world
            .delete_entity(
                self.camera
                    .take()
                    .expect("Expected camera entity to be set."),
            )
            .expect("Failed to delete camera entity.");
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for GamePlayState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting GamePlayState");
        self.initialize_dispatcher(&mut data.world);
        self.initialize_camera(&mut data.world);
        self.initialize_entities(&mut data.world);

        // TODO: create beat points from BeatMap
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.terminate_entities(&mut data.world);
        self.terminate_camera(&mut data.world);
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

    fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);

        // TODO: Probably render something on screen to say "the game is paused"
        // Should we also add an entity with a `Paused` component that indicates the paused state?
        if !self.paused {
            self.dispatcher.as_mut().unwrap().dispatch(&data.world.res);
        }

        // Map beatpoint visual components to beatmap logical beatpoints
        if self.progress_counter.as_ref().unwrap().is_complete() && !self.loaded {
            self.loaded = true;
            let mut beatpoints = Vec::<BeatPoint>::new();
            for (b,) in (&data.world.read_storage::<BeatPoint>(),).join() {
                beatpoints.push(b.clone());
            }
            beatpoints.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
            data.world.write_resource::<BeatMap>().beat_points = beatpoints.into();
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
