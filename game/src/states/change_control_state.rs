use amethyst::ecs::*;
use amethyst::input::is_close_requested;
use amethyst::renderer::{Event, PngFormat, Texture, TextureHandle};
use amethyst::shrev::EventChannel;
use amethyst::ui::{
    Anchor, FontAsset, TtfFormat, UiButton, UiButtonBuilder, UiEvent, UiEventType, UiImage, UiText,
    UiTransform,
};
use amethyst::{GameData, State, StateData, Trans};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

#[derive(Default, new)]
pub struct ChangeControlState {
    #[new(default)]
    ui_events: Option<ReaderId<UiEvent>>,

    #[new(default)]
    back_entity: Option<Entity>,
    #[new(default)]
    left_entity: Option<Entity>,
    #[new(default)]
    right_entity: Option<Entity>,
    #[new(default)]
    up_entity: Option<Entity>,
    #[new(default)]
    down_entity: Option<Entity>,
}

struct CleanupControl;
impl Component for CleanupControl {
    type Storage = VecStorage<CleanupControl>;
}

impl<'a, 'b> State<GameData<'a, 'b>> for ChangeControlState {
    fn on_start(&mut self, data: StateData<GameData>) {
        let mut world = data.world;
        world.register::<CleanupControl>();

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

        let (back, back_hover, back_press) = load_button_images(&mut world, "back");
        let back_button = UiButtonBuilder::new("controls_back_button", "")
            .with_position(85.0, 36.0)
            .with_image(back)
            .with_hover_image(back_hover)
            .with_press_image(back_press)
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        world
            .write_storage::<CleanupControl>()
            .insert(back_button, CleanupControl)
            .unwrap();
        self.back_entity = Some(back_button);

        let left_label = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Left".to_owned(),
                [0.0, 0.0, 0.0, 1.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "left_label".to_owned(),
                    Anchor::TopLeft,
                    75.0,
                    100.0,
                    -3.0,
                    75.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .with(CleanupControl)
            .build();

        let (change, change_hover, change_press) = load_button_images(&mut world, "change");

        let change_button = UiButtonBuilder::new("left_change_button", "")
            .with_position(200.0, 100.0)
            .with_image(change.clone())
            .with_hover_image(change_hover.clone())
            .with_press_image(change_press.clone())
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        world
            .write_storage::<CleanupControl>()
            .insert(change_button, CleanupControl)
            .unwrap();
        self.left_entity = Some(change_button);

        let right_label = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Right".to_owned(),
                [0.0, 0.0, 0.0, 1.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "right_label".to_owned(),
                    Anchor::TopLeft,
                    75.0,
                    150.0,
                    -3.0,
                    75.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .with(CleanupControl)
            .build();

        let change_button = UiButtonBuilder::new("right_change_button", "")
            .with_position(200.0, 150.0)
            .with_image(change.clone())
            .with_hover_image(change_hover.clone())
            .with_press_image(change_press.clone())
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        world
            .write_storage::<CleanupControl>()
            .insert(change_button, CleanupControl)
            .unwrap();
        self.right_entity = Some(change_button);

        let up_label = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Up".to_owned(),
                [0.0, 0.0, 0.0, 1.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "up_label".to_owned(),
                    Anchor::TopLeft,
                    75.0,
                    200.0,
                    -3.0,
                    75.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .with(CleanupControl)
            .build();

        let change_button = UiButtonBuilder::new("up_change_button", "")
            .with_position(200.0, 200.0)
            .with_image(change.clone())
            .with_hover_image(change_hover.clone())
            .with_press_image(change_press.clone())
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        world
            .write_storage::<CleanupControl>()
            .insert(change_button, CleanupControl)
            .unwrap();
        self.up_entity = Some(change_button);

        let down_label = world
            .create_entity()
            .with(UiText::new(
                font.clone(),
                "Down".to_owned(),
                [0.0, 0.0, 0.0, 1.0],
                25.0,
            ))
            .with(
                UiTransform::new(
                    "down_label".to_owned(),
                    Anchor::TopLeft,
                    75.0,
                    250.0,
                    -3.0,
                    75.0,
                    25.0,
                    2,
                ).as_transparent(),
            )
            .with(CleanupControl)
            .build();

        let change_button = UiButtonBuilder::new("down_change_button", "")
            .with_position(200.0, 250.0)
            .with_image(change.clone())
            .with_hover_image(change_hover.clone())
            .with_press_image(change_press.clone())
            .with_anchor(Anchor::TopLeft)
            .with_size(130.0, 32.0)
            .build_from_world(world);
        world
            .write_storage::<CleanupControl>()
            .insert(change_button, CleanupControl)
            .unwrap();
        self.down_entity = Some(change_button);

        let bottom_thing = world
            .write_resource::<AssetLoader>()
            .load(
                "ui/bottom_thing.png",
                PngFormat,
                Default::default(),
                &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                &mut world.write_resource(),
                &mut world.read_resource(),
            )
            .expect("Failed to load button press").clone();

        world
            .create_entity()
            .with(UiImage {
                texture: bottom_thing,
            })
            .with(
                UiTransform::new(
                    "bottom_thing".to_owned(),
                    Anchor::BottomRight,
                    -376.0,
                    -55.0,
                    -3.0,
                    752.0,
                    110.0,
                    2,
                ).as_transparent(),
            )
            .with(CleanupControl)
            .build();

        self.ui_events = Some(
            world
                .write_resource::<EventChannel<UiEvent>>()
                .register_reader(),
        );
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) {
            return Trans::Quit;
        }

        use systems::ChangeControl;
        use amethyst::input::InputHandler;

        if data.world.exec(
            |(channel, mut change, handler): (
                Read<EventChannel<UiEvent>>,
                Write<ChangeControl>,
                Read<InputHandler<String, String>>,
            )| {
                let value = handler.axis_value("X").unwrap();
                if value != 0.0 {
                    println!("hey X {}", value);
                }
                let value = handler.axis_value("Y").unwrap();
                if value != 0.0 {
                    println!("hey Y {}", value);
                }

                if let Some(reader) = self.ui_events.as_mut() {
                    for ev in channel.read(reader) {
                        match ev.event_type {
                            UiEventType::Click => {
                                if ev.target == self.back_entity.unwrap() {
                                    *change = ChangeControl::None;
                                    return true;
                                } else if ev.target == self.left_entity.unwrap() {
                                    println!("ah ok");
                                    *change = ChangeControl::Axis {
                                        name: "X".to_owned(),
                                        positive: true,
                                    };
                                } else if ev.target == self.right_entity.unwrap() {
                                    *change = ChangeControl::Axis {
                                        name: "X".to_owned(),
                                        positive: false,
                                    };
                                } else if ev.target == self.up_entity.unwrap() {
                                    *change = ChangeControl::Axis {
                                        name: "Y".to_owned(),
                                        positive: true,
                                    };
                                } else if ev.target == self.down_entity.unwrap() {
                                    *change = ChangeControl::Axis {
                                        name: "Y".to_owned(),
                                        positive: false,
                                    };
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                false
            },
        ) {
            cleanup(data.world);
            Trans::Pop
        } else {
            Trans::None
        }
    }

    fn update(&mut self, mut data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(data.world);
        Trans::None
    }
}

fn cleanup(world: &mut World) {
    let mut to_delete = world.exec(
        |(entities, cleanup_store): (Entities, ReadStorage<CleanupControl>)| {
            for (e, _) in (&*entities, &cleanup_store).join() {
                entities.delete(e);
            }
        },
    );
}

fn load_button_images(
    world: &mut World,
    name: &'static str,
) -> (TextureHandle, TextureHandle, TextureHandle) {
    let button = {
        world
            .write_resource::<AssetLoader>()
            .load(
                &format!("ui/{}.png", name),
                PngFormat,
                Default::default(),
                &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                &mut world.write_resource(),
                &mut world.read_resource(),
            )
            .expect("Failed to load button")
            .clone()
    };
    let button_hover = {
        world
            .write_resource::<AssetLoader>()
            .load(
                &format!("ui/{}_hover.png", name),
                PngFormat,
                Default::default(),
                &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                &mut world.write_resource(),
                &mut world.read_resource(),
            )
            .expect("Failed to load button hover")
            .clone()
    };
    let button_press = {
        world
            .write_resource::<AssetLoader>()
            .load(
                &format!("ui/{}_press.png", name),
                PngFormat,
                Default::default(),
                &mut world.write_resource::<AssetLoaderInternal<Texture>>(),
                &mut world.write_resource(),
                &mut world.read_resource(),
            )
            .expect("Failed to load button press")
            .clone()
    };
    (button, button_hover, button_press)
}
