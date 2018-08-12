use amethyst::ecs::{Component, DenseVecStorage, Entity,WriteStorage};
use amethyst::assets::{PrefabData,PrefabError};
use data::Direction;

#[derive(Deserialize, PartialEq, Clone,Serialize)]
pub struct BeatPoint {
    pub direction: Direction,
    pub time: f64,
}

impl Component for BeatPoint {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> PrefabData<'a> for BeatPoint {
    type SystemData = (
        WriteStorage<'a, BeatPoint>,
    );
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        system_data: &mut Self::SystemData,
        _entities: &[Entity],
    ) -> Result<(), PrefabError> {
        system_data.0.insert(entity, self.clone())?;
        Ok(())
    }
}