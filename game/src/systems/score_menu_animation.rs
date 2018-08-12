use amethyst::assets::AssetStorage;
use amethyst::audio::{Source, SourceHandle, output::Output};
use amethyst::core::Time;
use amethyst::ecs::*;
use amethyst::input::InputHandler;
use amethyst::renderer::VirtualKeyCode;
use amethyst::ui::{Anchor, UiText, UiTransform};
use amethyst_extra::{AssetLoader, AssetLoaderInternal};

use std::ops::Deref;

use data::ResultEntities;

enum State {
    Title(f32),
    Score(f32),
    Grade(f32),
}

pub struct ScoreMenuAnimation {
    state: State,
    boom: Option<SourceHandle>,
    rising: Option<SourceHandle>,
    ding: Option<SourceHandle>,
}

impl ScoreMenuAnimation {
    pub fn new() -> Self {
        Self {
            state: State::Title(0.0),
            boom: None,
            rising: None,
            ding: None,
        }
    }
}

impl<'a> System<'a> for ScoreMenuAnimation {
    type SystemData = (
        Write<'a, Option<ResultEntities>>,
        Read<'a, Time>,
        Option<Read<'a, Output>>,
        Read<'a, AssetStorage<Source>>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
        Write<'a, InputHandler<String, String>>,
    );

    fn run(
        &mut self,
        (
            mut entities_opt,
            time,
            audio_output,
            audio_storage,
            mut text_store,
            mut transf_store,
            input,
        ): Self::SystemData,
    ) {
        if let Some(ref entities) = *entities_opt {
            if input.key_is_down(VirtualKeyCode::Space) {
                // Skip animation
                if let Some(text) = text_store.get_mut(entities.title) {
                    text.color[3] = 1.0;
                }
                if let Some(text) = text_store.get_mut(entities.score_text) {
                    text.color[3] = 1.0;
                }
                if let Some(text) = text_store.get_mut(entities.score) {
                    text.color[3] = 1.0;
                    text.text = entities.target_score.to_string();
                }
                if let Some(icon) = transf_store.get_mut(entities.grade) {
                    icon.anchor = Anchor::Middle;
                }
                if let Some(text) = text_store.get_mut(entities.comment) {
                    text.color[3] = 1.0;
                }
                if let Some(butt) = transf_store.get_mut(entities.retry_button) {
                    butt.anchor = Anchor::Middle;
                }
                if let Some(butt) = transf_store.get_mut(entities.menu_button) {
                    butt.anchor = Anchor::Middle;
                }
                if let Some(ref ding) = self.ding {
                    play_sound(
                        ding,
                        &*audio_storage,
                        audio_output.as_ref().map(|o| o.deref()),
                    );
                }
                *entities_opt = None;
                self.state = State::Title(0.0);
            } else {
                let audio_storage = &*audio_storage;
                match self.state {
                    State::Title(ref mut x) => {
                        *x += time.delta_seconds();
                        if *x > 1.0 {
                            if let Some(ref boom) = self.boom {
                                play_sound(
                                    boom,
                                    audio_storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );
                            }
                            if let Some(text) = text_store.get_mut(entities.title) {
                                text.color[3] = 1.0;
                            }
                            self.state = State::Score(0.0);
                        }
                    }
                    State::Score(ref mut x) => {
                        *x += time.delta_seconds();
                        if *x > 1.0 {
                            if let Some(ref rising) = self.rising {
                                play_sound(
                                    rising,
                                    audio_storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );
                            }
                            if let Some(text) = text_store.get_mut(entities.score_text) {
                                text.color[3] = 1.0;
                            }
                            if let Some(text) = text_store.get_mut(entities.score) {
                                text.color[3] = 1.0;
                            }
                            self.state = State::Grade(0.0);
                        }
                    }
                    State::Grade(ref mut x) => {
                        *x += time.delta_seconds();
                        if let Some(text) = text_store.get_mut(entities.score) {
                            text.text = (((*x * 0.5).min(1.0) * entities.target_score as f32)
                                .floor() as u32)
                                .to_string();
                        }
                        if *x > 2.0 {
                            if let Some(ref ding) = self.ding {
                                play_sound(
                                    ding,
                                    audio_storage,
                                    audio_output.as_ref().map(|o| o.deref()),
                                );
                            }
                            if let Some(icon) = transf_store.get_mut(entities.grade) {
                                icon.anchor = Anchor::Middle;
                            }
                            if let Some(text) = text_store.get_mut(entities.comment) {
                                text.color[3] = 1.0;
                            }
                            if let Some(butt) = transf_store.get_mut(entities.retry_button) {
                                butt.anchor = Anchor::Middle;
                            }
                            if let Some(butt) = transf_store.get_mut(entities.menu_button) {
                                butt.anchor = Anchor::Middle;
                            }
                            *entities_opt = None;
                            self.state = State::Title(0.0);
                        }
                    }
                }
            }
        }
    }

    fn setup(&mut self, mut res: &mut Resources) {
        use amethyst::audio::OggFormat;

        Self::SystemData::setup(&mut res);
        let loader = res.fetch::<AssetLoader>();
        self.boom = Some(
            loader
                .load(
                    "audio/boom.ogg",
                    OggFormat,
                    (),
                    &mut res.fetch_mut::<AssetLoaderInternal<Source>>(),
                    &mut res.fetch_mut(),
                    &mut res.fetch_mut(),
                )
                .expect("Failed to load 'boom' sound effect"),
        );
        self.rising = Some(
            loader
                .load(
                    "audio/rising.ogg",
                    OggFormat,
                    (),
                    &mut res.fetch_mut::<AssetLoaderInternal<Source>>(),
                    &mut res.fetch_mut(),
                    &mut res.fetch_mut(),
                )
                .expect("Failed to load 'rising' sound effect"),
        );
        self.ding = Some(
            loader
                .load(
                    "audio/ding.ogg",
                    OggFormat,
                    (),
                    &mut res.fetch_mut::<AssetLoaderInternal<Source>>(),
                    &mut res.fetch_mut(),
                    &mut res.fetch_mut(),
                )
                .expect("Failed to load 'ding' sound effect"),
        );
    }
}

fn play_sound(sound: &SourceHandle, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(sound) {
            output.play_once(sound, 1.0);
        }
    }
}
