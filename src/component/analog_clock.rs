use Timer;
use std::f32::consts::PI;
use serde_json::{to_writer, Result};
use std::io::Write;

#[derive(Default)]
pub struct Component;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub seconds: (f32, f32),
    pub minutes: (f32, f32),
    pub hours: (f32, f32),
}

impl State {
    pub fn write_json<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        to_writer(&mut writer, self)
    }
}

impl Component {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn state(&self, timer: &Timer) -> State {
        let time = timer.current_time()[timer.current_timing_method()].unwrap_or_default();
        let total_seconds = time.total_seconds();
        let seconds = total_seconds / 60.0;
        let minutes = seconds / 60.0;
        let hours = minutes / 12.0;
        State {
            seconds: percentage_to_coord(seconds, 0.9),
            minutes: percentage_to_coord(minutes, 0.8),
            hours: percentage_to_coord(hours, 0.5),
        }
    }
}

fn percentage_to_coord(p: f64, factor: f32) -> (f32, f32) {
    let (sin, cos) = (p as f32 * 2.0 * PI).sin_cos();
    let x = 0.5 * factor * sin + 0.5;
    let y = -0.5 * factor * cos + 0.5;
    (x, y)
}
