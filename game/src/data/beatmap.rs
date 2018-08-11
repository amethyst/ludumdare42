use amethyst::audio::Source;
use amethyst::assets::Handle;

use data::BeatPoint;

#[derive(Deserialize)]
pub struct BeatMapData {
    pub name: String,
    pub music_path: String,
    pub audio_offset: f64,
    pub beat_points: Vec<BeatPoint>,
}

pub struct BeatMap {
    pub name: String,
    pub music: Handle<Source>,
    pub audio_offset: f64,
    pub beat_points: Vec<BeatPoint>,
}