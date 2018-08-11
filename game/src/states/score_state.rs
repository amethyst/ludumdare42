use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::prelude::Builder;
use amethyst::renderer::{ElementState, Event, PngFormat, Texture, VirtualKeyCode};
use amethyst::ui::{Anchor, FontAsset, TtfFormat, UiButtonBuilder, UiImage, UiText, UiTransform};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use data::{GameplayResult, GameplayStatus, HitResult, ResultEntities};

/// Where the player is running out of space
#[derive(Default)]
pub struct ScoreState;

impl ScoreState {
    /// Creates a new ScoreState.
    pub fn new() -> Self {
        ScoreState
    }
}

fn compute_score(result: &GameplayResult) -> u32 {
    let mut score = 0;
    for (_, h) in &result.results {
        score += match h {
            HitResult::Hit => 1000,
            HitResult::MissKey => 100,
            HitResult::MissEarly | HitResult::MissLate => 10,
        };
    }
    return score;
}

enum Grade {
    A,
    B,
    C,
    S,
    F,
}

fn compute_grade(result: &GameplayResult) -> Grade {
    let mut successes = 0;
    for (_, h) in &result.results {
        successes += match h {
            HitResult::Hit => 1,
            _ => 0,
        };
    }
    let ratio = successes as f32 / result.results.len() as f32;
    if ratio < 0.50 {
        Grade::C
    } else if ratio < 0.80 {
        Grade::B
    } else if ratio < 0.97 {
        Grade::A
    } else {
        Grade::S
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
            .expect("Failed to load font");

        let result = world.read_resource::<GameplayResult>().clone();

        let (title_text, grade) = if result.status == GameplayStatus::Completed {
            ("Congratulations!".to_owned(), compute_grade(&result))
        } else {
            ("Oh no!".to_owned(), Grade::F)
        };

        let (grade, comment) = match grade {
            Grade::C => (
                {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            "ui/C_rank.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load rank C")
                        .clone()
                },
                "That's a start".to_owned(),
            ),
            Grade::B => (
                {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            "ui/B_rank.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load rank B")
                        .clone()
                },
                "Okay!".to_owned(),
            ),
            Grade::A => (
                {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            "ui/A_rank.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load rank A")
                        .clone()
                },
                "Not bad!".to_owned(),
            ),
            Grade::S => (
                {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            "ui/S_rank.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load rank S")
                        .clone()
                },
                "Awesome!".to_owned(),
            ),
            Grade::F => (
                {
                    world
                        .write_resource::<AssetLoader>()
                        .load(
                            "ui/F_rank.png",
                            PngFormat,
                            Default::default(),
                            &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                            &mut world.write_resource(),
                            &mut world.read_resource(),
                        )
                        .expect("Failed to load rank F")
                        .clone()
                },
                "".to_owned(),
            ),
        };

        let title = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                title_text,
                [1.0, 1.0, 1.0, 0.0],
                50.0,
            ))
            .with(
                UiTransform::new(
                    "title".to_owned(),
                    Anchor::Middle,
                    20.0,
                    -100.0,
                    -3.0,
                    500.0,
                    50.0,
                    2,
                ).as_transparent(),
            )
            .build();

        let score_text = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Score".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "score_text".to_owned(),
                    Anchor::Middle,
                    20.0,
                    -20.0,
                    -3.0,
                    500.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .build();

        let score = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "0".to_owned(),
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "score".to_owned(),
                    Anchor::Middle,
                    100.0,
                    -20.0,
                    -3.0,
                    500.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .build();

        let comment = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                comment,
                [1.0, 1.0, 1.0, 0.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "comment".to_owned(),
                    Anchor::Middle,
                    20.0,
                    20.0,
                    -3.0,
                    500.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .build();

        let grade = world
            .create_entity()
            .with(UiImage { texture: grade })
            .with(
                UiTransform::new(
                    "grade".to_owned(),
                    Anchor::MiddleRight,
                    110.0,
                    40.0,
                    -3.0,
                    175.0,
                    175.0,
                    2,
                ).as_transparent(),
            )
            .build();

        let retry = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/retry.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load retry")
                .clone()
        };
        let retry_hover = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/retry_hover.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load retry hover")
                .clone()
        };
        let retry_press = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/retry_press.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load retry press")
                .clone()
        };

        let retry_button = UiButtonBuilder::new("score_retry_button", "")
            .with_position(-170.0, 110.0)
            .with_image(retry)
            .with_hover_image(retry_hover)
            .with_press_image(retry_press)
            .with_font_size(25.0)
            .with_tab_order(0)
            .with_anchor(Anchor::BottomMiddle)
            .with_font(font.clone())
            .with_size(100.0, 32.0)
            .build_from_world(world);

        let menu = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/menu.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load menu")
                .clone()
        };
        let menu_hover = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/menu_hover.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load menu hover")
                .clone()
        };
        let menu_press = {
            world
                .write_resource::<AssetLoader>()
                .load(
                    "ui/menu_press.png",
                    PngFormat,
                    Default::default(),
                    &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                    &mut world.write_resource(),
                    &mut world.read_resource(),
                )
                .expect("Failed to load menu press")
                .clone()
        };

        let menu_button = UiButtonBuilder::new("score_menu_button", "")
            .with_position(-50.0, 110.0)
            .with_image(menu)
            .with_hover_image(menu_hover)
            .with_press_image(menu_press)
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

            target_score: 12345//compute_score(&result),
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
