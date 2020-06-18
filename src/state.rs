use super::source_impl::Sources;

#[derive(Debug)]
pub struct State {
    sources: Sources,
}

impl State {
    pub fn new(sources: Sources) -> Self {
        State { sources }
    }

    pub fn sources(&self) -> &Sources {
        &self.sources
    }
}
