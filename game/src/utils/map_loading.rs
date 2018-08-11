use std::fs;
use amethyst_extra::AssetLoader;
use std::fs::File;
use amethyst::ecs::World;
use amethyst::audio::Mp3Format;
use std::io::Read;
use ron::de::from_str;


use data::*;

pub fn list_directory(dir: &String) -> Vec<String> {
    fs::read_dir(dir)
        .expect(&*format!("Failed to read directory {}", dir))
        .map(|e| {
            String::from(
                e.expect("Failed to read file path.")
                    .path()
                    //.file_stem()
                    .to_str()
                    .unwrap(),
            )
        })
        .collect()
}

pub fn list_beatmaps(asset_loader: &AssetLoader) -> Vec<String> {
    if let Some(path) = asset_loader.resolve_path("maps") {
        return list_directory(&path);
    }
    vec![]
}

pub fn load_beatmap(name: String, world: &mut World) -> Option<BeatMap> {
    if let Some(path) = &world.read_resource::<AssetLoader>().resolve_path(&format!("maps/{}/map.ron",name)) {
        // don't fuck with file permissions thanks
        let mut file = File::open(path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        match from_str::<BeatMapData>(&contents) {
            Ok(data) => {
                //load audio
                if let Some(audio_handle) =
                    world.read_resource::<AssetLoader>()
                        .load(&data.music_path, Mp3Format, (), &mut world.write_resource(), &mut world.write_resource(), &world.read_resource()){
                    return Some(BeatMap{
                        name: data.name,
                        music: audio_handle,
                        audio_offset: data.audio_offset,
                        beat_points: data.beat_points,
                    });
                }else{
                    error!("Failed to load audio handle for {}", data.name);
                }

            },
            Err(err) => error!("Failed to deserialize map data: {:?}",err),
        }
    }else{
        error!("Failed to find map {}",name);
    }
    None
}