use amethyst::assets::{PrefabData, PrefabError};
use amethyst::ecs::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RotatingObject {
    pub velocity: f32,
}

impl Component for RotatingObject {
    type Storage = VecStorage<FallingObject>;
}

impl<'a> PrefabData<'a> for RotatingObject {
    type SystemData = (WriteStorage<'a, RotatingObject>);
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<(), PrefabError> {
        data.insert(
            entity,
            self.clone(),
        ).unwrap();
        Ok(())
    }
}
