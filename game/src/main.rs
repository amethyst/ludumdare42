#![feature(nll)]

extern crate amethyst;
extern crate amethyst_extra;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
extern crate rand;
extern crate ron;
#[macro_use]
extern crate serde;

use amethyst::animation::*;
use amethyst::assets::*;
use amethyst::audio::*;
use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::*;
use amethyst::ecs::*;
use amethyst::input::*;
use amethyst::prelude::*;
use amethyst::renderer::mouse::set_mouse_cursor_none;
use amethyst::renderer::*;
use amethyst::ui::*;
use amethyst::utils::scene::BasicScenePrefab;
use amethyst::Result;
use amethyst_extra::*;

use std::env;

mod data;
mod states;
mod systems;
mod utils;

pub use data::*;
pub use states::*;
pub use systems::*;
pub use utils::*;

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let asset_loader =
        AssetLoader::new(&format!("{}/assets", get_working_dir()).to_string(), "base");
    let display_config_path = asset_loader.resolve_path("config/display.ron").unwrap();
    let key_bindings_path = asset_loader.resolve_path("config/input.ron").unwrap();

    let config = DisplayConfig::load(&display_config_path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.05, 0.1, 0.2, 1.0], 1.0)
            .with_pass(DrawSprite::new().with_transparency(ColorMask::all(), ALPHA, None))
            .with_pass(DrawUi::new()),
    );

    let game_data_builder = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?,
        )?.with(
            FollowMouseSystem::<String, String>::default(),
            "follow_mouse",
            &[],
        ).with_bundle(TransformBundle::new().with_dep(&["follow_mouse"]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(AnimationBundle::<u32, Material>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?.with_bundle(AudioBundle::new(|music: &mut ::utils::Music| music.next()))?
        .with(
            PrefabLoaderSystem::<SpriteScenePrefab>::default(),
            "",
            &[],
        ).with(TimedDestroySystem, "timed_destroy", &[])
        .with(NormalOrthoCameraSystem::default(), "aspect_ratio", &[])
        .with(
            VisibilitySortingSystem::new(),
            "visibility",
            &["transform_system"],
        ).with(ChangeControlListener, "change_control", &[])
        .with(MapSelectionUiEventHandlerSystem::new(), "map_selection_ui_event_handler_system", &["ui_button_system"])
        .with(ScoreMenuAnimation::new(), "score_menu_animation", &[])
        .with(MakeObjectsFall, "make_objects_fall", &[])
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    let resources_directory = format!("");
    Application::build(resources_directory, MapSelectionState::new())?
        .with_resource(asset_loader)
        .with_resource(::utils::Music::default())
        .with_resource(AssetLoaderInternal::<Texture>::new())
        .with_resource(AssetLoaderInternal::<FontAsset>::new())
        .with_resource(AssetLoaderInternal::<amethyst::audio::Source>::new())
        .with_resource::<Option<ResultEntities>>(None)
        .with_resource::<GameplayCommand>(GameplayCommand::BackToMenu)
        .build(game_data_builder)?
        .run();
    Ok(())
}
