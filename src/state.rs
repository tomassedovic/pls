#[derive(Default)]
pub struct State {
    pub selected_index: usize,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
