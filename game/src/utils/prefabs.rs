use amethyst::assets::{AssetStorage, Loader, PrefabData, PrefabError, ProgressCounter};
use amethyst::core::Transform;
use amethyst::renderer::*;
use amethyst::ecs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteSheetPrefab {
    pub id: u64,
    pub texture: (u64, TexturePrefab<TextureFormat>),
    pub sprites: Vec<Sprite>,
}

impl<'a> PrefabData<'a> for SpriteSheetPrefab {
    type SystemData = (
        Write<'a, MaterialTextureSet>,
        <TexturePrefab<TextureFormat> as PrefabData<'a>>::SystemData,
        Write<'a, SpriteSheetSet>,
        ReadExpect<'a, Loader>,
        Read<'a, AssetStorage<SpriteSheet>>,
    );
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        Ok(())
    }

    fn trigger_sub_loading(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData,
    ) -> Result<bool, PrefabError> {
        let mut ret = false;
        match system_data.0.handle(self.texture.0) {
            Some(handle) => (),
            None => {
                ret = self.texture
                    .1
                    .trigger_sub_loading(progress, &mut system_data.1)?;
                if let TexturePrefab::Handle(ref handle) = self.texture.1 {
                    system_data.0.insert(self.texture.0, handle.clone());
                }
            }
        };
        if let None = system_data.2.handle(self.id) {
            let spritesheet = SpriteSheet {
                texture_id: self.texture.0,
                sprites: self.sprites.clone(),
            };
            let handle = system_data
                .3
                .load_from_data(spritesheet, progress, &system_data.4);
            system_data.2.insert(self.id, handle);
            ret = true;
        }
        Ok(ret)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteRenderPrefab {
    pub sheet: u64,
    pub sprite_number: usize,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl<'a> PrefabData<'a> for SpriteRenderPrefab {
    type SystemData = (Read<'a, SpriteSheetSet>, WriteStorage<'a, SpriteRender>);
    type Result = ();
    fn load_prefab(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        system_data
            .1
            .insert(
                entity,
                SpriteRender {
                    sprite_sheet: system_data.0.handle(self.sheet).unwrap().clone(),
                    sprite_number: self.sprite_number,
                    flip_horizontal: self.flip_horizontal,
                    flip_vertical: self.flip_vertical,
                },
            )
            .map(|_| ())
    }
}

/// Sprite scene prefab
///
/// Usage:
///
/// ```rust,ignore
/// let prefab_handle = data.world.exec(|loader: PrefabLoader<SpriteScenePrefab>| {
///     loader.load("prefab.ron", RonFormat, (), ())
/// });
/// data.world.create_entity().with(prefab_handle).build();
/// ```
///
/// ```ron,ignore
/// #![enable(implicit_some)]
/// Prefab (
///     entities: [
///         (
///             data: (
///                 sprite_sheets: [
///                     (
///                         id: 0,
///                         texture: (0, File("texture.png", Png, ())),
///                         sprites: [
///                             (
///                                 left: 0,
///                                 right: 1,
///                                 bottom: 0,
///                                 top: 1,
///                             ),
///                         ],
///                     ),
///                 ],
///             ),
///         ),
///         (
///             data: (
///                 sprite: (
///                     sheet: 0,
///                     sprite_number: 0,
///                     flip_horizontal: false,
///                     flip_vertical: false,
///                 ),
///                 transform: (
///                     translation: (
///                         x: 4,
///                         y, 1,
///                         z: 0,
///                     ),
///                 ),
///             ),
///         ),
///     ]
/// )
/// ```
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct SpriteScenePrefab {
    sprite_sheets: Vec<SpriteSheetPrefab>,
    sprite: Option<SpriteRenderPrefab>,
    transform: Option<Transform>,
}

impl Default for SpriteScenePrefab {
    fn default() -> Self {
        SpriteScenePrefab {
            sprite_sheets: Vec::new(),
            sprite: None,
            transform: None,
        }
    }
}

impl<'a> PrefabData<'a> for SpriteScenePrefab {
    type SystemData = (
        <SpriteSheetPrefab as PrefabData<'a>>::SystemData,
        <SpriteRenderPrefab as PrefabData<'a>>::SystemData,
        <Transform as PrefabData<'a>>::SystemData,
    );
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        for sprite_sheet in &self.sprite_sheets {
            sprite_sheet.load_prefab(entity, &mut system_data.0, entities)?;
        }
        self.sprite
            .load_prefab(entity, &mut system_data.1, entities)?;
        self.transform
            .load_prefab(entity, &mut system_data.2, entities)?;
        Ok(())
    }

    fn trigger_sub_loading(
        &mut self,
        progress: &mut ProgressCounter,
        system_data: &mut Self::SystemData,
    ) -> Result<bool, PrefabError> {
        let mut ret = false;
        for sprite_sheet in &mut self.sprite_sheets {
            if sprite_sheet.trigger_sub_loading(progress, &mut system_data.0)? {
                ret = true;
            }
        }
        if self.sprite
            .trigger_sub_loading(progress, &mut system_data.1)?
        {
            ret = true;
        }
        if self.transform
            .trigger_sub_loading(progress, &mut system_data.2)?
        {
            ret = true;
        }
        Ok(ret)
    }
}
