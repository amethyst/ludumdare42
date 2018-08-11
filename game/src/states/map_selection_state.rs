use amethyst::ecs::prelude::*;
use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::ui::{Anchor, FontAsset, FontHandle, TtfFormat, UiButtonBuilder};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use utils::list_beatmaps;
use GamePlayState;

/// Where the player chooses which song to play
#[derive(Default)]
pub struct MapSelectionState {
    buttons: Vec<Entity>,
    font: Option<FontHandle>,
}

impl MapSelectionState {
    /// Creates a new MapSelectionState.
    pub fn new() -> Self {
        MapSelectionState {
            buttons: Vec::new(),
            font: None,
        }
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

        let font = self.font.as_ref().unwrap();
        self.buttons = beatmaps
            .iter()
            .enumerate()
            .map(|(i, beatmap)| {
                UiButtonBuilder::new(beatmap, beatmap)
                    .with_position(0.0, 40.0 + 100.0 * (i as f32 + 1.0))
                    .with_text_color([0.7; 4])
                    .with_hover_text_color([1.0; 4])
                    .with_press_text_color([0.5; 4])
                    .with_font_size(25.0)
                    .with_tab_order(i as i32)
                    .with_anchor(Anchor::TopMiddle)
                    .with_font(font.clone())
                    .build_from_world(world)
            })
            .collect::<Vec<Entity>>();
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
