use amethyst::ecs::prelude::*;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiEvent, UiEventType};

use BeatmapButton;
use MapSelectionEvent;

/// System that processes `UiEvent`s and generates `MapSelectionEvent`s.
#[derive(Debug, Default)]
pub struct MapSelectionUiEventHandlerSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl MapSelectionUiEventHandlerSystem {
    pub fn new() -> Self {
        Default::default()
    }
}

type MapSelectionUiEventHandlerSystemData<'s> = (
    Read<'s, EventChannel<UiEvent>>,
    Write<'s, EventChannel<MapSelectionEvent>>,
    ReadStorage<'s, BeatmapButton>,
);

impl<'s> System<'s> for MapSelectionUiEventHandlerSystem {
    type SystemData = MapSelectionUiEventHandlerSystemData<'s>;

    fn run(&mut self, (ui_events, mut map_selection_events, beatmap_buttons): Self::SystemData) {
        for ev in ui_events.read(self.reader_id.as_mut().unwrap()) {
            if let UiEvent {
                event_type: UiEventType::Click,
                target: entity,
            } = *ev
            {
                if let Some(beatmap_button) = beatmap_buttons.get(entity) {
                    let beatmap = beatmap_button.beatmap.clone();
                    let map_selection_event = MapSelectionEvent::Select(beatmap);
                    map_selection_events.single_write(map_selection_event);
                }
            }
        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(res.fetch_mut::<EventChannel<UiEvent>>().register_reader());
    }
}
