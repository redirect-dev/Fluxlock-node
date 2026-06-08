use fluxlock_core::types::Validator;

// =========================
// 👑 AUTHORITY SUCCESSION
// =========================
pub fn run_authority_succession(
    validators: &mut Vec<Validator>,
) {
    let snapshot =
        validators.clone();

    for validator in validators.iter_mut() {

        if !validator.elected_authority {
            continue;
        }

        if validator.effective_authority > 100.0 {
            continue;
        }

        let mut successor_id = None;
        let mut best_score = 0.0;

        for peer in snapshot.iter() {

            if peer.id == validator.id {
                continue;
            }

            let score =
                peer.effective_authority
                + peer.leadership_score
                + peer.continuity_memory_score
                + (
                    peer.historical_consensus_accuracy
                    * 100.0
                );

            if score > best_score {

                best_score = score;

                successor_id =
                    Some(peer.id);
            }
        }

        if let Some(new_leader) =
            successor_id
        {
            validator.authority_successor =
                Some(new_leader);

            validator.succession_count += 1;
        }
    }
}