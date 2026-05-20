use fluxlock_core::types::{
    Validator,
};

// =========================
// 🌐 DISTANCE
// =========================
fn topology_distance(
    a: &Validator,
    b: &Validator,
) -> f64 {

    let cluster_delta =
        (a.topology_cluster as i32
        - b.topology_cluster as i32)
        .abs() as f64;

    1.0 + cluster_delta
}

// =========================
// 🌐 PEER GOVERNANCE ENGINE
// =========================
pub fn propagate_peer_governance(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    for validator in
        validators.iter_mut()
    {

        let mut support = 0.0;

        let mut opposition = 0.0;

        let mut immune_support = 0.0;

        let mut fracture_pressure = 0.0;

        let mut rehabilitation_pressure = 0.0;

        let mut entropy_field = 0.0;

        let mut healing_field = 0.0;

        let mut resonance_gain = 0.0;

        let mut trust_gravity = 0.0;

        for peer in snapshot.iter() {

            if peer.id == validator.id {
                continue;
            }

            let distance =
                topology_distance(
                    validator,
                    peer
                );

            let proximity =
                1.0 / distance;

            // =========================
            // 🌟 HEALTHY SUPPORT
            // =========================
            if peer.status == "healthy" {

                support +=
                    peer.peer_reputation
                    * 0.003
                    * proximity;

                healing_field +=
                    peer.healing_wave
                    * 0.004
                    * proximity;

                validator.trust +=
                    peer.stabilization_power
                    * 0.0015
                    * proximity;

                validator.consensus_pressure -=
                    0.015
                    * proximity;
            }

            // =========================
            // 🛡 IMMUNE REINFORCEMENT
            // =========================
            if peer.immune_response > 20.0 {

                immune_support +=
                    peer.immune_strength
                    * 0.005
                    * proximity;

                validator.resilience_score +=
                    0.02
                    * proximity;

                validator.drift -=
                    0.01
                    * proximity;
            }

            // =========================
            // ☠ FRACTURE PROPAGATION
            // =========================
            if peer.status == "fractured" {

                fracture_pressure +=
                    peer.fracture_severity
                    * 0.015
                    * proximity;

                entropy_field +=
                    peer.entropy_output
                    * 0.01
                    * proximity;

                validator.drift +=
                    peer.instability_radius
                    * 0.005
                    * proximity;

                validator.peer_reputation *=
                    0.999;
            }

            // =========================
            // 🔁 RECOVERY CASCADE
            // =========================
            if peer.status == "recovering" {

                rehabilitation_pressure +=
                    peer.rehabilitation_score
                    * 0.003
                    * proximity;

                validator.trust +=
                    0.015
                    * proximity;
            }

            // =========================
            // 🔴 QUARANTINE PRESSURE
            // =========================
            if peer.quarantine_level > 25.0 {

                opposition +=
                    peer.quarantine_level
                    * 0.005
                    * proximity;

                validator.consensus_pressure +=
                    0.03
                    * proximity;
            }

            // =========================
            // 🌌 TRUST GRAVITY
            // =========================
            trust_gravity +=
                peer.trust_gravity
                * 0.002
                * proximity;

            // =========================
            // 🌊 RESONANCE
            // =========================
            resonance_gain +=
                peer.resonance_score
                * 0.003
                * proximity;
        }

        // =========================
        // 🌐 GOVERNANCE
        // =========================
        validator.governance_weight +=
            support * 0.0005;

        validator.governance_weight -=
            opposition * 0.0005;

        validator.governance_weight =
            validator
                .governance_weight
                .clamp(0.1, 10.0);

        // =========================
        // 🛡 IMMUNITY
        // =========================
        validator.immune_response +=
            immune_support * 0.001;

        validator.immune_strength +=
            immune_support * 0.0005;

        validator.immune_response =
            validator
                .immune_response
                .clamp(0.0, 1000.0);

        // =========================
        // ☠ FRACTURE
        // =========================
        validator.fracture_severity +=
            fracture_pressure * 0.001;

        validator.consensus_pressure +=
            fracture_pressure * 0.002;

        // =========================
        // 🌊 ENTROPY FIELD
        // =========================
        validator.regional_pressure +=
            entropy_field * 0.001;

        validator.drift +=
            entropy_field * 0.0005;

        // =========================
        // 🌟 HEALING FIELD
        // =========================
        validator.healing_wave +=
            healing_field * 0.0005;

        validator.drift -=
            healing_field * 0.0004;

        // =========================
        // 🔁 RECOVERY
        // =========================
        validator.rehabilitation_score +=
            rehabilitation_pressure
            * 0.001;

        // =========================
        // 🌌 RESONANCE
        // =========================
        validator.resonance_score +=
            resonance_gain * 0.0005;

        // =========================
        // 🌠 TRUST GRAVITY
        // =========================
        validator.trust_gravity +=
            trust_gravity * 0.0005;

        // =========================
        // 🌐 CONSENSUS EVOLUTION
        // =========================
        if validator.consensus_pressure > 40.0 {

            validator.peer_votes_invalid += 1;

        } else {

            validator.peer_votes_valid += 1;
        }

        // =========================
        // 🧬 SOCIAL ISOLATION
        // =========================
        if validator.fracture_severity > 100.0 {

            validator.network_accepted =
                false;

            validator.status =
                "fractured".into();

            validator.isolation_events += 1;
        }

        // =========================
        // 🌟 REHABILITATION
        // =========================
        if validator.rehabilitation_score > 200.0
        && validator.drift < 15.0
        && validator.chain_valid
        {

            validator.network_accepted =
                true;

            validator.status =
                "healthy".into();
        }

        // =========================
        // 🌌 INFLUENCE EVOLUTION
        // =========================
        validator.network_influence_score +=
            validator.peer_reputation
            * 0.0002;

        validator.network_influence_score -=
            validator.fracture_severity
            * 0.0003;

        validator.network_influence_score +=
            validator.resonance_score
            * 0.0002;

        validator.network_influence_score =
            validator
                .network_influence_score
                .clamp(0.0, 1000.0);

        // =========================
        // 🌊 ECOLOGY STABILIZATION
        // =========================
        validator.drift =
            validator.drift.max(0.0);

        validator.regional_pressure =
            validator
                .regional_pressure
                .clamp(0.0, 1000.0);

        validator.healing_wave =
            validator
                .healing_wave
                .clamp(0.0, 1000.0);

        validator.entropy_output =
            validator
                .entropy_output
                .clamp(0.0, 1000.0);
    }
}