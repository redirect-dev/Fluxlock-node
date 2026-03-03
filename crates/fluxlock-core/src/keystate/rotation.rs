use super::types::KeyState;

/// Determines if rotation is allowed at this tick
pub fn should_rotate(state: &KeyState, tick_index: u64) -> bool {
    tick_index > state.activated_at_tick
}

/// Performs deterministic key rotation
pub fn rotate_key(
    previous: &KeyState,
    new_pubkey: Vec<u8>,
    tick_index: u64,
) -> Result<KeyState, String> {

    let mut new_state = previous.clone();

    new_state.current_pubkey = new_pubkey;
    new_state.key_epoch += 1;
    new_state.activated_at_tick = tick_index;

    // No commitment logic for now
    new_state.next_pubkey_commitment = None;

    Ok(new_state)
}
