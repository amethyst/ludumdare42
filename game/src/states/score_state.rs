use amethyst::ecs::*;
use amethyst::input::{get_key, is_close_requested, is_key_down};
use amethyst::prelude::Builder;
use amethyst::renderer::{ElementState, Event, PngFormat, Texture, VirtualKeyCode};
use amethyst::shrev::EventChannel;
use amethyst::ui::{Anchor, FontAsset, TtfFormat, UiButtonBuilder, UiEvent, UiEventType, UiImage,
                   UiText, UiTransform};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use data::{GameplayCommand, GameplayResult, GameplayStatus, HitResult, ResultEntities};

/// Where the player is running out of space
#[derive(Default)]
pub struct ScoreState {
    ui_events: Option<ReaderId<UiEvent>>,
    menu_button: Option<Entity>,
    retry_button: Option<Entity>,
}

impl ScoreState {
    /// Creates a new ScoreState.
    pub fn new() -> Self {
        ScoreState {
            ui_events: None,
            menu_button: None,
            retry_button: None,
        }
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
    if ratio < 0.40 {
        Grade::C
    } else if ratio < 0.70 {
        Grade::B
    } else if ratio < 0.97 {
        Grade::A
    } else {
        Grade::S
    }
}

struct CleanupScore;
impl Component for CleanupScore {
    type Storage = VecStorage<CleanupScore>;
}

impl<'a, 'b> State<GameData<'a, 'b>> for ScoreState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        debug!("Starting ScoreState");

        let world = data.world;

        world.register::<CleanupScore>();

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

        //let (title_text, grade) = ("Congratulations!".to_owned(), Grade::S);

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
            .with(CleanupScore)
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
            .with(CleanupScore)
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
            .with(CleanupScore)
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
            .with(CleanupScore)
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
            .with(CleanupScore)
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

        world
            .write_storage::<CleanupScore>()
            .insert(retry_button, CleanupScore)
            .unwrap();

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

        world
            .write_storage::<CleanupScore>()
            .insert(menu_button, CleanupScore)
            .unwrap();

        *world.write_resource::<Option<ResultEntities>>() = Some(ResultEntities {
            title,
            score_text,
            score,
            grade,
            comment,
            menu_button,
            retry_button,

            target_score: compute_score(&result),
        });

        self.menu_button = Some(menu_button);
        self.retry_button = Some(retry_button);

        self.ui_events = Some(
            world
                .write_resource::<EventChannel<UiEvent>>()
                .register_reader(),
        );
    }

    fn handle_event(
        &mut self,
        mut data: StateData<GameData>,
        event: Event,
    ) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) {
            return Trans::Quit;
        }

        Trans::None
    }

    fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);

        let mut new_command: Option<GameplayCommand> = None;

        for ev in data.world
            .read_resource::<EventChannel<UiEvent>>()
            .read(&mut self.ui_events.as_mut().unwrap())
        {
            match ev.event_type {
                UiEventType::Click => {
                    if ev.target == self.retry_button.unwrap() {
                        new_command = Some(GameplayCommand::Retry);
                    } else {
                        new_command = Some(GameplayCommand::BackToMenu);
                    }
                    break;
                }
                _ => {}
            }
        }

        if let Some(command) = new_command {
            cleanup(&mut data.world);
            *data.world.write_resource::<GameplayCommand>() = command;
            Trans::Pop
        } else {
            Trans::None
        }
    }
}

fn cleanup(world: &mut World) {
    world.exec(
        |(entities, cleanup_store): (Entities, ReadStorage<CleanupScore>)| {
            for (e, _) in (&*entities, &cleanup_store).join() {
                entities.delete(e).unwrap();
            }
        },
    );
}
