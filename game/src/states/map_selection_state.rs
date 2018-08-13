use amethyst::core::{GlobalTransform, Time, Transform};
use amethyst::ecs::prelude::*;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::renderer::{Camera, PngFormat, Projection, Texture};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};

use amethyst::core::cgmath::{Matrix4, Ortho, Vector3};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{
    Anchor, FontAsset, TtfFormat, UiButton, UiButtonBuilder, UiEvent, UiEventType, UiImage, UiText,
    UiTransform, FontHandle
};
use amethyst::{GameData, State, StateData, Trans};

use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use super::map_selection::*;
use data::BeatPoint;
use utils::{list_beatmaps, load_beatmap};
use GamePlayState;
use MapSelectionEvent;

/// Where the player chooses which song to play
#[derive(Default, new)]
pub struct MapSelectionState {
    #[new(default)]
    map_selection_event_reader: Option<ReaderId<MapSelectionEvent>>,
    #[new(default)]
    buttons: Vec<Entity>,
    #[new(default)]
    font: Option<FontHandle>,
    #[new(value = "false")]
    cam_init: bool,
    #[new(default)]
    controls_button: Option<Entity>,
    #[new(default)]
    ui_events: Option<ReaderId<UiEvent>>,
}

impl MapSelectionState {
    fn initialize_map_selection_event_channel(&mut self, world: &mut World) {
        let mut map_selection_event_channel = EventChannel::<MapSelectionEvent>::with_capacity(20);
        let reader_id = map_selection_event_channel.register_reader();
        self.map_selection_event_reader.get_or_insert(reader_id);

        world.add_resource(map_selection_event_channel);
    }

    fn terminate_map_selection_event_channel(&mut self) {
        self.map_selection_event_reader.take();
    }

    /// Reloads the beatmaps and recreates the menu.
    fn reload_menu(&mut self, world: &mut World) {
        let beatmaps = list_beatmaps(&mut world.write_resource::<AssetLoader>());
        debug!("Beatmaps: {:?}", beatmaps);

        if self.font.is_none() {
            let font = &mut world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/square.ttf",
                    TtfFormat,
                    (),
                    &mut world.write_resource::<AssetLoaderInternal<FontAsset>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load font.");
            self.font = Some(font.clone());
        }

        let controls = world
            .write_resource::<AssetLoader>()
            .load(
                "ui/controls.png",
                PngFormat,
                Default::default(),
                &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                &mut world.write_resource(),
                &mut world.read_resource(),
            )
            .expect("Failed to load retry")
            .clone();

        let controls_button = UiButtonBuilder::new("controls_button", "")
            .with_position(85.0, 36.0)
            .with_image(controls)
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        self.controls_button = Some(controls_button);

        let font = self.font.as_ref().unwrap();
        // let mut index = 0;
        self.buttons = beatmaps
            .iter()
            .enumerate()
            .map(|(i, beatmap)| {
                // index += 1;
                debug!("Index...{}", i as f32);
                let img = {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            &format!("maps/level{}/level.png", (i as f32 + 1.0)),
                            // "ui/level1.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load retry")
                        .clone()
                };
                let imghover = {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            &format!("maps/level{}/levelhover.png", (i as f32 + 1.0)),
                            // "ui/level1hover.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load retry")
                        .clone()
                };

                let entity = UiButtonBuilder::new(beatmap, "")
                    .with_position(0.0,-400.0 + 100.0 * ((i as f32 + 1.0) * 2.0))
                    // .with_text_color([0.7; 4])
                    // .with_hover_text_color([1.0; 4])
                    // .with_press_text_color([0.5; 4])
                    // .with_font_size(50.0)
                    .with_size(150.0, 150.0)
                    .with_tab_order(i as i32)
                    .with_anchor(Anchor::Middle)
                    .with_font(font.clone())
                    .with_image(img)
                    .with_hover_image(imghover)
                    .build_from_world(world);

                let mut beatmap_button_storage = world.write_storage::<BeatmapButton>();
                beatmap_button_storage
                    .insert(entity, BeatmapButton::new(beatmap.clone()))
                    .expect("Failed to insert beatmap_button component.");

                entity
            })
            .collect::<Vec<Entity>>();
        self.buttons.push(controls_button);

        self.ui_events = Some(
            world
                .write_resource::<EventChannel<UiEvent>>()
                .register_reader(),
        );
    }

    fn clear_menu(&mut self, world: &mut World) {
        self.buttons.drain(..).for_each(|button| {
            world
                .delete_entity(button)
                .expect("Failed to delete button.");
        });
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for MapSelectionState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting MapSelectionState");
        data.world.register::<BeatPoint>();

        // We don't have an EntryState, so this needs to go here.
        if !self.cam_init {
            self.cam_init = true;
            // The Z coordinate of the camera is how far along it should be before it faces the
            // entities. Anything greater than this will be culled.
            let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, 100.0));
            let global_transform = GlobalTransform(translation);

            let camera = data
                .world
                .create_entity()
                .with(Camera::from(Projection::Orthographic(Ortho {
                    left: 0.0,
                    right: 1.0,
                    top: 1.0,
                    bottom: 0.0,
                    near: 0.0,
                    far: 2000.,
                })))
                .with(global_transform)
                .build();
        }

        self.initialize_map_selection_event_channel(&mut data.world);
        self.reload_menu(&mut data.world);
    }

    fn on_resume(&mut self, mut data: StateData<GameData>) {
        debug!("Resuming MapSelectionState");
        self.reload_menu(&mut data.world);
    }

    fn on_pause(&mut self, mut data: StateData<GameData>) {
        self.clear_menu(&mut data.world);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.clear_menu(&mut data.world);
        self.terminate_map_selection_event_channel();
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        if data
            .world
            .exec(|channel: Read<EventChannel<UiEvent>>| {
                if let Some(reader) = self.ui_events.as_mut() {
                    for ev in channel.read(reader) {
                        match ev.event_type {
                            UiEventType::Click => {
                                if ev.target == self.controls_button.unwrap() {
                                    return true;
                                }
                            }
                            _ => { }
                        }
                    }
                }
                false
            })
        {
            use states::ChangeControlState;
            Trans::Push(Box::new(ChangeControlState::new()))
        } else {
            Trans::None
        }
    }

    fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);

        // sorry for bad memory management, but this is a game jam
        let beatmap_name = {
            let map_selection_event_channel = data
                .world
                .read_resource::<EventChannel<MapSelectionEvent>>();

            let mut reader_id = self
                .map_selection_event_reader
                .as_mut()
                .expect("Expected map_selection_event_reader to be set");

            let mut storage_iterator = map_selection_event_channel.read(&mut reader_id);
            storage_iterator.next().map(|event| match *event {
                MapSelectionEvent::Select(ref beatmap_name) => beatmap_name.clone(),
            })
        };

        if let Some(beatmap_name) = beatmap_name {
            debug!("Beatmap selected: {}", &beatmap_name);
            let mut beatmap =
                load_beatmap(beatmap_name, &mut data.world).expect("Failed to load beatmap :(");
            // Maps should start in 3 seconds from now.
            beatmap.runtime_start =
                data.world.read_resource::<Time>().absolute_time_seconds() + 3.0;
            data.world.add_resource(beatmap);

            Trans::Push(Box::new(GamePlayState::new()))
        } else {
            Trans::None
        }
    }
}
