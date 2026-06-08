use fluxlock_core::types::Validator;

// =========================
// ⚔ AUTHORITY CHALLENGES
// =========================
pub fn run_authority_challenges(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    for validator in validators.iter_mut() {

        // =========================
        // 👑 ONLY LEADERS
        // =========================
        if !validator.elected_authority {
            continue;
        }

        // =========================
        // ⚠ CHALLENGE CONDITIONS
        // =========================
        if validator.authority_legitimacy > 85.0
            &&
            validator.governance_term < 500
        {
            continue;
        }

        let mut strongest_candidate =
            None;

        let mut best_score = 0.0;

        // =========================
        // 🔍 FIND CHALLENGER
        // =========================
        for peer in snapshot.iter() {

            if peer.id == validator.id {
                continue;
            }

            if peer.effective_authority
                > best_score
            {

                best_score =
                    peer.effective_authority;

                strongest_candidate =
                    Some(peer.id);
            }
        }

        // =========================
        // ⚔ CHALLENGE
        // =========================
        if let Some(_challenger_id) =
            strongest_candidate
        {

            if best_score >
                validator.effective_authority
            {

                validator
                    .authority_challenges_lost += 1;

                validator
                    .authority_demotions += 1;

                validator
                    .active_challenges += 1;

                // =========================
                // ⚖ CHALLENGE PENALTIES
                // =========================
                validator
                    .authority_legitimacy *= 0.90;

                validator
                    .authority_points *= 0.95;

                validator
                    .network_influence_score *= 0.98;

                println!(
                    "⚔ AUTHORITY CHALLENGE LOST | Validator {}",
                    validator.id
                );
            }
        }
    }
}