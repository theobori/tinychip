use std::time;

/// The clock timer state
pub enum ClockState {
    Finish,
    Progress,
}

/// Easier way to manage time
#[derive(Clone, Copy)]
pub struct Clock {
    /// micros duration
    pub cooldown: time::Duration,
    /// Couting milliseconds
    pub instant: time::Instant,
}

impl Clock {
    pub fn new(micros: u64) -> Self {
        Self {
            cooldown: time::Duration::from_micros(micros),
            instant: time::Instant::now(),
        }
    }

    /// Reset the instant
    pub fn reset(&mut self) -> &mut Self {
        self.instant = time::Instant::now();

        self
    }

    /// Try to reset the instant
    pub fn try_reset(&mut self) -> bool {
        if *self == ClockState::Finish {
            self.reset();

            true
        } else {
            false
        }
    }
}

impl PartialEq<ClockState> for Clock {
    fn eq(&self, state: &ClockState) -> bool {
        let elapsed = self.instant.elapsed().as_micros();
        let cooldown = self.cooldown.as_micros();

        match state {
            ClockState::Finish => elapsed >= cooldown,
            ClockState::Progress => elapsed < cooldown,
        }
    }
}
