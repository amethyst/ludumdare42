use amethyst::ecs::Entity;

pub struct ResultEntities {
    pub title: Entity,
    pub score_text: Entity,
    pub score: Entity,
    pub comment: Entity,
    pub grade: Entity,
    pub menu_button: Entity,
    pub retry_button: Entity,

    pub target_score: u32,
}
