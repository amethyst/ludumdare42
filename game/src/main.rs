extern crate amethyst;
extern crate amethyst_extra;
#[macro_use]
extern crate log;
extern crate rand;

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
use amethyst::Result;
use amethyst_extra::*;
use amethyst::utils::scene::BasicScenePrefab;

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
    
    let asset_loader = AssetLoader::new(
        &format!("{}/assets", working_dir()).to_string(),
        "base",
    );
    let display_config_path = asset_loader.resolve_path("config/display.ron").unwrap();
    let key_bindings_path = asset_loader.resolve_path("config/input.ron").unwrap();

    let game_data_builder = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?
        )?
        .with(
            FollowMouseSystem::<String, String>::default(),
            "follow_mouse",
            &[],
        )
        .with_bundle(TransformBundle::new().with_dep(&[
            "follow_mouse",
        ]))?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_bundle(AnimationBundle::<u32, Material>::new(
            "animation_control_system",
            "sampler_interpolation_system",
        ))?
        .with_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))?
        .with(PrefabLoaderSystem::<BasicScenePrefab<Vec<PosTex>>>::default(), "", &[])
        .with(TimedDestroySystem, "timed_destroy", &[])
        .with(NormalOrthoCameraSystem::default(), "aspect_ratio", &[])
        .with(VisibilitySortingSystem::new(), "visibility", &["transform_system"])
        .with_basic_renderer(display_config_path, DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None), true)?;
    let resources_directory = format!("");
    Application::build(resources_directory, TestState)?
        .with_resource(asset_loader)
        .build(game_data_builder)?
        .run();
    Ok(())
}
