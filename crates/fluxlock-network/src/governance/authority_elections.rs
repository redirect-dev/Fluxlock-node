use fluxlock_core::types::Validator;

// =========================
// 🗳 AUTHORITY ELECTION
// =========================
pub fn run_authority_election(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    let election_threshold =
        (
            snapshot.len() as u64
            * 70
        )
        / 100;

    for validator in validators.iter_mut() {

        let mut votes = 0u64;

        for peer in snapshot.iter() {

            if peer.id == validator.id {
                continue;
            }

            let candidate_score =

                validator.trust

                + validator.continuity_reputation

                + validator.adaptive_reputation

                + validator.continuity_memory_score

                + (
                    validator.authority_points
                    * 0.01
                )

                + (
                    validator
                        .historical_consensus_accuracy
                    * 100.0
                );

            let peer_threshold =

                peer.trust

                + peer.continuity_reputation

                + (
                    peer.authority_points
                    * 0.005
                );

            if candidate_score
                > peer_threshold
            {
                votes += 1;
            }
        }

        validator.election_votes_received =
            votes;
    }

    for validator in validators.iter_mut() {

        if validator.election_votes_received
            >= election_threshold
        {

            validator.elected_authority = true;

            validator.election_wins += 1;

            validator.governance_term = 100;

            validator.authority_points += 100.0;

            validator.leadership_score += 5.0;

            validator.network_influence_score += 5.0;

        } else {

            validator.elected_authority = false;

            // =========================
            // ⚖ ELECTION LOSS COST
            // =========================
            validator.authority_points *=
                0.995;

            validator.leadership_score *=
                0.999;

            validator.network_influence_score *=
                0.999;
        }
    }
}