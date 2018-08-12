use amethyst::assets::{AssetStorage, Loader, PrefabData, PrefabError, ProgressCounter};
use amethyst::core::Transform;
use amethyst::ecs::*;
use amethyst::renderer::*;

use data::*;
use amethyst_extra::RemovalPrefab;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteSheetPrefab {
    pub id: u64,
    pub texture: (u64, TexturePrefab<TextureFormat>),
    pub sprites: Vec<Sprite>,
}

impl SpriteSheetPrefab {
    fn do_load_prefab<'a>(
        &self,
        _entity: Entity,
        _mtl_tex_set: &mut MaterialTextureSet,
        _tex_system_data: &mut <TexturePrefab<TextureFormat> as PrefabData<'a>>::SystemData,
        _sprite_sheet_set: &mut SpriteSheetSet,
        _loader: &Loader,
        _sprite_sheet_store: &AssetStorage<SpriteSheet>,
        _entities: &[Entity],
    ) -> Result<(), PrefabError> {
        Ok(())
    }

    fn do_sub_loading<'a>(
        &mut self,
        progress: &mut ProgressCounter,
        mtl_tex_set: &mut MaterialTextureSet,
        mut tex_system_data: &mut <TexturePrefab<TextureFormat> as PrefabData<'a>>::SystemData,
        sprite_sheet_set: &mut SpriteSheetSet,
        loader: &Loader,
        sprite_sheet_store: &AssetStorage<SpriteSheet>,
    ) -> Result<bool, PrefabError> {
        let mut ret = false;
        match mtl_tex_set.handle(self.texture.0) {
            Some(handle) => (),
            None => {
                ret = self
                    .texture
                    .1
                    .trigger_sub_loading(progress, &mut tex_system_data)?;
                if let TexturePrefab::Handle(ref handle) = self.texture.1 {
                    mtl_tex_set.insert(self.texture.0, handle.clone());
                }
            }
        };
        if let None = sprite_sheet_set.handle(self.id) {
            let spritesheet = SpriteSheet {
                texture_id: self.texture.0,
                sprites: self.sprites.clone(),
            };
            let handle = loader.load_from_data(spritesheet, progress, &sprite_sheet_store);
            sprite_sheet_set.insert(self.id, handle);
            ret = true;
        }
        Ok(ret)
    }
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
        (
            mtl_tex_set,
            tex_system_data,
            sprite_sheet_set,
            loader,
            sprite_sheet_store,
        ): &mut Self::SystemData,
        entities: &[Entity],
) -> Result<(), PrefabError>{
        self.do_load_prefab(
            entity,
            mtl_tex_set,
            tex_system_data,
            sprite_sheet_set,
            loader,
            sprite_sheet_store,
            entities,
        )
    }

    fn trigger_sub_loading(
        &mut self,
        progress: &mut ProgressCounter,
        (
            mtl_tex_set,
            tex_system_data,
            sprite_sheet_set,
            loader,
            sprite_sheet_store,
        ): &mut Self::SystemData,
) -> Result<bool, PrefabError>{
        self.do_sub_loading(
            progress,
            mtl_tex_set,
            tex_system_data,
            sprite_sheet_set,
            loader,
            sprite_sheet_store,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpriteRenderPrefab {
    pub sheet: u64,
    pub sprite_number: usize,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl SpriteRenderPrefab {
    fn do_load_prefab<'a>(
        &self,
        entity: Entity,
        sprite_sheet_set: &SpriteSheetSet,
        sprite_renders: &mut WriteStorage<'a, SpriteRender>,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        sprite_renders
            .insert(
                entity,
                SpriteRender {
                    sprite_sheet: sprite_sheet_set.handle(self.sheet).unwrap().clone(),
                    sprite_number: self.sprite_number,
                    flip_horizontal: self.flip_horizontal,
                    flip_vertical: self.flip_vertical,
                },
            )
            .map(|_| ())
    }

    fn do_sub_loading<'a>(
        &mut self,
        _progress: &mut ProgressCounter,
        _sprite_sheet_set: &SpriteSheetSet,
        _sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    ) -> Result<bool, PrefabError> {
        Ok(false)
    }
}

impl<'a> PrefabData<'a> for SpriteRenderPrefab {
    type SystemData = (Read<'a, SpriteSheetSet>, WriteStorage<'a, SpriteRender>);
    type Result = ();
    fn load_prefab(
        &self,
        entity: Entity,
        (sprite_sheet_set, sprite_renders): &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        self.do_load_prefab(entity, &sprite_sheet_set, sprite_renders, entities)
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
    beat_point: Option<BeatPoint>,
    removal: Option<RemovalPrefab<i32>>,
}

impl Default for SpriteScenePrefab {
    fn default() -> Self {
        SpriteScenePrefab {
            sprite_sheets: Vec::new(),
            sprite: None,
            transform: None,
            beat_point: None,
            removal: None,
        }
    }
}

impl<'a> PrefabData<'a> for SpriteScenePrefab {
    type SystemData = (
        (
            Write<'a, MaterialTextureSet>,
            <TexturePrefab<TextureFormat> as PrefabData<'a>>::SystemData,
            Write<'a, SpriteSheetSet>,
            ReadExpect<'a, Loader>,
            Read<'a, AssetStorage<SpriteSheet>>,
            WriteStorage<'a, SpriteRender>,
        ),
        <Transform as PrefabData<'a>>::SystemData,
        <BeatPoint as PrefabData<'a>>::SystemData,
        <RemovalPrefab<i32> as PrefabData<'a>>::SystemData,
    );
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        (
            (
                mtl_tex_set,
                tex_system_data,
                sprite_sheet_set,
                loader,
                sprite_sheet_store,
                sprite_renders,
            ),
            transform_system_data,
            beatpoint_system_data,
            removal_system_data,
        ): &mut Self::SystemData,
        entities: &[Entity],
    ) -> Result<(), PrefabError> {
        for sprite_sheet in &self.sprite_sheets {
            sprite_sheet.do_load_prefab(
                entity,
                mtl_tex_set,
                tex_system_data,
                sprite_sheet_set,
                loader,
                sprite_sheet_store,
                entities,
            )?;
        }
        if self.sprite.is_some() {
            self.sprite.as_ref().unwrap().do_load_prefab(
                entity,
                sprite_sheet_set,
                sprite_renders,
                entities,
            )?;
        }
        self.transform
            .load_prefab(entity, transform_system_data, entities)?;
        if self.beat_point.is_some() {
            self.beat_point.as_ref().unwrap().load_prefab(
                entity,
                beatpoint_system_data,
                entities,
            )?;
        }
        if self.removal.is_some() {
            self.removal.as_ref().unwrap().load_prefab(
                entity,
                removal_system_data,
                entities,
            )?;
        }
        Ok(())
    }

    fn trigger_sub_loading(
        &mut self,
        progress: &mut ProgressCounter,
        (
            (
                mtl_tex_set,
                tex_system_data,
                sprite_sheet_set,
                loader,
                sprite_sheet_store,
                sprite_renders,
            ),
            transform_system_data,
            beatpoint_system_data,
            removal_system_data,
        ): &mut Self::SystemData,
    ) -> Result<bool, PrefabError> {
        let mut ret = false;
        for sprite_sheet in &mut self.sprite_sheets {
            if sprite_sheet.do_sub_loading(
                progress,
                mtl_tex_set,
                tex_system_data,
                sprite_sheet_set,
                loader,
                sprite_sheet_store,
            )? {
                ret = true;
            }
        }
        if self.sprite.is_some() {
            self.sprite.as_mut().unwrap().do_sub_loading(
                progress,
                sprite_sheet_set,
                sprite_renders,
            )?;
            ret = true;
        }
        if self
            .transform
            .trigger_sub_loading(progress, transform_system_data)?
        {
            ret = true;
        }
        /*if self.beatpoint.is_some() {
            self.beatpoint.as_ref().unwrap().do_sub_loading(
                progress,
                beatpoint_system_data
            )?;
            ret = true;
        }*/
        Ok(ret)
    }
}
