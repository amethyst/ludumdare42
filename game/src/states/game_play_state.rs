use amethyst::assets::*;
use amethyst::audio::{AudioSink, Mp3Format, Source as AudioSource, SourceHandle};
use amethyst::core::cgmath::{Matrix4, Ortho, Vector3};
use amethyst::core::{GlobalTransform, Transform};
use amethyst::ecs::prelude::*;
use amethyst::input::{get_key, is_close_requested};
use amethyst::renderer::{
    Camera, ElementState, Event, Projection, ScreenDimensions, SpriteRender, SpriteSheetSet,
    VirtualKeyCode,
};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::*;

use std::collections::VecDeque;

use data::*;
use systems::PlayerMovementSystem;
use utils::{Music, SpriteScenePrefab};
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
    /// The handle to the music asset
    #[new(default)]
    music: Option<SourceHandle>,
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
        let mut progress_counter = ProgressCounter::default();

        // === Player === //
        let player_prefab_path = world
            .read_resource::<AssetLoader>()
            .resolve_path("prefabs/player.ron")
            .expect("Please ensure prefabs/player.ron exists inside assets/<directory>/");

        // Load the player!
        let prefab_handle = world.exec(|loader: PrefabLoader<SpriteScenePrefab>| {
            loader.load(player_prefab_path, RonFormat, (), &mut progress_counter)
        });

        let player = world
            .create_entity()
            .with(Player::default())
            // .with(Transform::default())
            .with(prefab_handle)
            .build();

        self.entities.push(player);

        // === BeatMap === //

        // Find prefab file to load
        let beatmap_name;
        let scene_path;
        let beat_points;
        {
            let beatmap = &world.read_resource::<BeatMap>();
            beatmap_name = beatmap.name.clone();
            scene_path = world
                .read_resource::<AssetLoader>()
                .resolve_path(&format!("maps/{}/scene.ron", beatmap_name))
                .expect(&format!(
                    "Please ensure map.ron::name == name of the folder containing map.ron for map {}",
                    beatmap_name
                ));
            beat_points = beatmap.beat_points.clone();
        }

        // BeatPoints
        let mut beatpoint_entities = beat_points
            .into_iter()
            .map(|beat_point| {
                let mut transform = Transform::default();
                transform.translation =
                    Vector3::new((beat_point.time * 220.0 + 50.) as f32, 140., 1.);
                world
                    .create_entity()
                    .with(beat_point)
                    .with(transform)
                    .with(GlobalTransform::default())
                    .build()
            })
            .collect::<Vec<Entity>>();

        self.entities.extend(beatpoint_entities);

        // === Background prefab === //

        // Load the map background!
        let prefab_handle = world.exec(|loader: PrefabLoader<SpriteScenePrefab>| {
            // might fail with abs path??
            loader.load(scene_path, RonFormat, (), &mut progress_counter)
        });
        let background_entity = world.create_entity().with(prefab_handle).build();
        self.entities.push(background_entity);

        let music = world.exec(
            |(resolver, loader, sources): (
                ReadExpect<AssetLoader>,
                ReadExpect<Loader>,
                Read<AssetStorage<AudioSource>>,
            )| {
                let path = resolver.resolve_path(&format!("maps/{}/audio.mp3", beatmap_name))
                // TODO use some fallback
                .unwrap_or_else(|| "assets/base/maps/test/audio.mp3".to_owned());
                loader.load(path, Mp3Format, (), &mut progress_counter, &sources)
            },
        );
        self.music = Some(music);

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
            // @jojo:
            //
            // how were you going to define BeatPoints?
            // I put them inside the BeatMap definition for now and spawn the entities from that.
            // This loop makes it look like you wanted to store the BeatPoint definition somewhere
            // else and then modify the BeatMap.
            //
            // Note, I attach sprite renders to the beat point entities after loading is complete
            // because the arrows are loaded by the scene prefab. It's a hack, but I have to go soon
            //
            // - az
            //
            // let mut beatpoints = Vec::<BeatPoint>::new();
            // for (b,) in (&data.world.read_storage::<BeatPoint>(),).join() {
            //     beatpoints.push(b.clone());
            // }
            // beatpoints.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
            // data.world.write_resource::<BeatMap>().beat_points = beatpoints.into();

            data.world.exec(
                |(entities, beat_points, sprite_sheet_set, mut sprite_renders): (
                    Entities,
                    ReadStorage<BeatPoint>,
                    Read<SpriteSheetSet>,
                    WriteStorage<SpriteRender>,
                )| {
                    let (up, down, left, right) = (
                        sprite_sheet_set
                            .handle(1)
                            .expect("No sprite sheet handle found for up arrow."),
                        sprite_sheet_set
                            .handle(2)
                            .expect("No sprite sheet handle found for down arrow."),
                        sprite_sheet_set
                            .handle(3)
                            .expect("No sprite sheet handle found for left arrow."),
                        sprite_sheet_set
                            .handle(4)
                            .expect("No sprite sheet handle found for right arrow."),
                    );
                    for (entity, beat_point) in (&*entities, &beat_points).join() {
                        let sprite_sheet = match beat_point.direction {
                            Direction::Up => up.clone(),
                            Direction::Down => down.clone(),
                            Direction::Left => left.clone(),
                            Direction::Right => right.clone(),
                        };
                        let sprite_render = SpriteRender {
                            sprite_sheet,
                            sprite_number: 0,
                            flip_horizontal: false,
                            flip_vertical: false,
                        };
                        sprite_renders.insert(entity, sprite_render).expect(
                            "Failed to attach sprite render component to beat point entity",
                        );
                    }

                    info!("Attached sprite renders to beat points.");
                },
            );

            // Play music
            data.world
                .add_resource(Music::new(self.music.as_ref().unwrap().clone()));
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
