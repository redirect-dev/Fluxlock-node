use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Epoch {
    pub number: u64,
}

pub trait EpochProvider {
    fn current_epoch(&self) -> Epoch;
}

/// Fixed duration epoch provider (deterministic)
pub struct FixedEpochProvider {
    pub epoch_duration_secs: u64,
}

impl FixedEpochProvider {
    pub fn new(epoch_duration_secs: u64) -> Self {
        Self { epoch_duration_secs }
    }

    fn unix_time_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

impl EpochProvider for FixedEpochProvider {
    fn current_epoch(&self) -> Epoch {
        let now = Self::unix_time_secs();
        let epoch_number = now / self.epoch_duration_secs;

        Epoch {
            number: epoch_number,
        }
    }
}