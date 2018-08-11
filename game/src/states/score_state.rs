use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::prelude::Builder;
use amethyst::renderer::{ElementState, Event, VirtualKeyCode};
use amethyst::ui::{Anchor, FontAsset, TtfFormat, UiButtonBuilder, UiText, UiCreator, UiTransform};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use data::{GameplayResult, GameplayStatus, ResultEntities};

/// Where the player is running out of space
#[derive(Default)]
pub struct ScoreState;

impl ScoreState {
    /// Creates a new ScoreState.
    pub fn new() -> Self {
        ScoreState
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for ScoreState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting ScoreState");

        let world = data.world;

        let font = world
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

        /*world.exec(|mut creator: UiCreator| {
            creator.create("assets/base/prefabs/example.ron", ());
        });*/

        /*let result = world.read_resource::<GameplayResult>().clone();
        let title_text = if result.status == GameplayStatus::Completed {
            "Congratulations!".to_owned()
        } else {
            "Oh no!".to_owned()
        };*/

        let title_text = "MISSION FAILED".to_owned();
        
        let title = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                title_text,
                [1.0, 1.0, 1.0, 0.0],
                50.0,
            ))
            .with(UiTransform::new(
                "title".to_owned(),
                Anchor::Middle,
                20.0,
                -100.0,
                -3.0,
                500.0,
                50.0,
                2,
            ).as_transparent())
            .build();

        let score_text = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Score".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(UiTransform::new(
                "score_text".to_owned(),
                Anchor::Middle,
                20.0,
                -20.0,
                -3.0,
                500.0,
                25.0,
                2,
            ).as_transparent())
            .build();

        let score = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "0".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(UiTransform::new(
                "score".to_owned(),
                Anchor::Middle,
                100.0,
                -20.0,
                -3.0,
                500.0,
                25.0,
                2,
            ).as_transparent())
            .build();

        let comment = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Okay!".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(UiTransform::new(
                "comment".to_owned(),
                Anchor::Middle,
                20.0,
                20.0,
                -3.0,
                500.0,
                25.0,
                2,
            ).as_transparent())
            .build();

        let grade = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "X".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                300.0,
            ))
            .with(UiTransform::new(
                "grade".to_owned(),
                Anchor::Middle,
                275.0,
                50.0,
                -3.0,
                500.0,
                300.0,
                2,
            ).as_transparent())
            .build();

        let retry_button = UiButtonBuilder::new("score_retry_button", "Retry")
            .with_position(-170.0, 110.0)
            .with_text_color([0.0, 0.0, 0.0, 1.0])
            .with_hover_text_color([1.0; 4])
            .with_press_text_color([0.5; 4])
            .with_font_size(25.0)
            .with_tab_order(0)
            .with_anchor(Anchor::BottomMiddle)
            .with_font(font.clone())
            .with_size(100.0, 32.0)
            .build_from_world(world);

        let menu_button = UiButtonBuilder::new("score_menu_button", "Menu")
            .with_position(-50.0, 110.0)
            .with_text_color([0.0, 0.0, 0.0, 1.0])
            .with_hover_text_color([1.0; 4])
            .with_press_text_color([0.5; 4])
            .with_font_size(25.0)
            .with_tab_order(1)
            .with_anchor(Anchor::BottomMiddle)
            .with_font(font.clone())
            .with_size(100.0, 32.0)
            .build_from_world(world);

        *world.write_resource::<Option<ResultEntities>>() = Some(ResultEntities {
            title,
            score_text,
            score,
            grade,
            comment,
            menu_button,
            retry_button,

            target_score: 1234567,
        });
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
        Trans::None
    }
}
