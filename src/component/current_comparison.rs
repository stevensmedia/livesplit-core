use Timer;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub comparison: String,
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut state = State::default();
        state.update(self, timer);
        state
    }
}

impl State {
    pub fn update(&mut self, _component: &Component, timer: &Timer) {
        self.text.clear();
        self.text.push_str("Comparing Against");

        self.comparison.clear();
        self.comparison.push_str(timer.current_comparison());
    }

    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
