use amethyst::assets::Handle;
use amethyst::audio::Source;

use data::BeatPoint;
use std::collections::VecDeque;

#[derive(Deserialize)]
pub struct BeatMapData {
    pub name: String,
    pub music_path: String,
    pub audio_offset: f64,
    //pub beat_points: VecDeque<BeatPoint>,
}

pub struct BeatMap {
    pub name: String,
    pub music: Handle<Source>,
    pub audio_offset: f64,
    pub beat_points: VecDeque<BeatPoint>,
    /// This needs to be changed to Time::absolute_time_seconds() + 3 when inserting the map into resources and starting the level.
    pub runtime_start: f64,
}
