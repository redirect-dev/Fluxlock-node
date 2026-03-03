use super::types::KeyState;

#[derive(Debug)]
pub enum KeyStateViolation {
    EpochRegression,
}

pub fn validate_transition(
    previous: &KeyState,
    next: &KeyState,
    _tick_index: u64,
) -> Result<(), KeyStateViolation> {

    // Only enforce epoch monotonicity for now
    if next.key_epoch < previous.key_epoch {
        return Err(KeyStateViolation::EpochRegression);
    }

    Ok(())
}
