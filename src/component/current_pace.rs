use {Timer, TimerPhase, comparison};
use serde_json::{to_writer, Result};
use std::io::Write;
use std::fmt::Write as FmtWrite;
use analysis::current_pace;
use time_formatter::{Regular, TimeFormatter, Accuracy};

#[derive(Default)]
pub struct Component {
    settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub comparison_override: Option<String>,
    pub accuracy: Accuracy,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            comparison_override: None,
            accuracy: Accuracy::Seconds,
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub text: String,
    pub time: String,
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn state(&self, timer: &Timer) -> State {
        let mut state = State::default();
        state.update(self, timer);
        state
    }
}

impl State {
    pub fn update(&mut self, component: &Component, timer: &Timer) {
        let comparison = component
            .settings
            .comparison_override
            .as_ref()
            .and_then(|c| timer.run().comparisons().find(|&rc| c == rc))
            .unwrap_or_else(|| timer.current_comparison());

        let mut current_pace = current_pace::calculate(timer, comparison);

        self.text.clear();

        match comparison {
            comparison::personal_best::NAME => self.text.push_str("Current Pace"),
            comparison::best_segments::NAME => self.text.push_str("Best Possible Time"),
            comparison::worst_segments::NAME => self.text.push_str("Worst Possible Time"),
            comparison::average_segments::NAME => self.text.push_str("Predicted Time"),
            comparison => write!(self.text, "Current Pace ({})", comparison).unwrap(),
        }

        if timer.current_phase() == TimerPhase::NotRunning &&
            self.text.starts_with("Current Pace")
        {
            current_pace = None;
        }

        self.time.clear();
        write!(
            self.time,
            "{}",
            Regular::with_accuracy(component.settings.accuracy).format(current_pace)
        ).unwrap();
    }

    pub fn write_json<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(writer, self)
    }
}
