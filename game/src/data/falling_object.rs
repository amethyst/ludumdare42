use amethyst::assets::{PrefabData, PrefabError};
use amethyst::ecs::*;

pub struct FallingObject {
    pub ground_level: f32,
    pub velocity: f32,
    pub falling: bool,
    pub trigger_time: f32,
}

impl FallingObject {
    pub fn new(ground_level: f32, velocity: f32, trigger_time: f32) -> Self {
        Self {
            ground_level,
            velocity,
            trigger_time,
            falling: false,
        }
    }
}

impl Component for FallingObject {
    type Storage = VecStorage<FallingObject>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FallingObjectPrefab {
    pub ground_level: f32,
    pub velocity: f32,
    pub trigger_time: f32,
}

impl<'a> PrefabData<'a> for FallingObjectPrefab {
    type SystemData = (WriteStorage<'a, FallingObject>);
    type Result = ();

    fn load_prefab(
        &self,
        entity: Entity,
        data: &mut Self::SystemData,
        _: &[Entity],
    ) -> Result<(), PrefabError> {
        data.insert(
            entity,
            FallingObject::new(self.ground_level, self.velocity, self.trigger_time),
        ).unwrap();
        Ok(())
    }
}
