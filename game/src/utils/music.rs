use amethyst::audio::SourceHandle;

#[derive(Default)]
pub struct Music {
    source: Option<SourceHandle>,
}

impl Music {
    pub fn new(source: SourceHandle) -> Self {
        Music {
            source: Some(source),
        }
    }

    pub fn next(&self) -> Option<SourceHandle> {
        self.source.as_ref().map(Clone::clone)
    }
}
