use std::time::Instant;
use std::ops::Sub;
use TimeSpan;
use atomic_clock_synchronization::drift;

lazy_static! {
    static ref BEGINNING_INSTANT: Instant = Instant::now();
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeStamp(TimeSpan);

impl TimeStamp {
    pub fn now() -> Self {
        TimeStamp(TimeSpan::from(BEGINNING_INSTANT.elapsed()) / drift())
    }
}

impl Sub for TimeStamp {
    type Output = TimeSpan;

    fn sub(self, rhs: TimeStamp) -> TimeSpan {
        self.0 - rhs.0
    }
}

impl Sub<TimeSpan> for TimeStamp {
    type Output = TimeStamp;

    fn sub(self, rhs: TimeSpan) -> TimeStamp {
        TimeStamp(self.0 - rhs)
    }
}
