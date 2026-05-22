use fluxlock_core::types::{
    Validator,
};

// =========================
// 🌐 CONTINUITY MESH
// =========================
pub fn propagate_continuity_mesh(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    for validator in
        validators.iter_mut()
    {

        let mut stabilization = 0.0;

        let mut entropy = 0.0;

        let mut resonance = 0.0;

        let mut neighbors = 0.0;

        for peer in snapshot.iter()
        {

            if peer.id == validator.id {

                continue;
            }

            // =========================
            // 🌐 SAME TOPOLOGY CLUSTER
            // =========================
            if peer.topology_cluster
                == validator.topology_cluster
            {

                neighbors += 1.0;

                stabilization +=
                    peer.healing_wave
                    * 0.015;

                stabilization +=
                    peer.trust_gravity
                    * 0.002;

                entropy +=
                    peer.entropy_output
                    * 0.010;

                resonance +=
                    peer.resonance_score
                    * 0.005;
            }
        }

        // =========================
        // 🌊 APPLY STABILIZATION
        // =========================
        validator.stabilization_power +=
            stabilization;

        // =========================
        // ☠ APPLY ENTROPY
        // =========================
        validator.consensus_pressure +=
            entropy;

        validator.fracture_severity +=
            entropy * 0.05;

        // =========================
        // 🌌 RESONANCE
        // =========================
        validator.resonance_score +=
            resonance;

        validator.continuity_anchor_strength +=
            stabilization * 0.01;

        // =========================
        // 🌐 DENSITY EFFECT
        // =========================
        validator.regional_pressure +=
            neighbors * 0.10;

        // =========================
        // 🛡 IMMUNE RESPONSE
        // =========================
        if validator.chain_valid {

            validator.healing_wave += 0.25;

            validator.trust_gravity += 0.10;

        } else {

            validator.entropy_output += 0.50;
        }

        // =========================
        // 🔒 LIMITS
        // =========================
        validator.resonance_score =
            validator
                .resonance_score
                .clamp(0.0, 5000.0);

        validator.stabilization_power =
            validator
                .stabilization_power
                .clamp(0.0, 5000.0);

        validator.consensus_pressure =
            validator
                .consensus_pressure
                .clamp(0.0, 5000.0);

        validator.fracture_severity =
            validator
                .fracture_severity
                .clamp(0.0, 1000.0);

        validator.healing_wave =
            validator
                .healing_wave
                .clamp(0.0, 1000.0);

        validator.entropy_output =
            validator
                .entropy_output
                .clamp(0.0, 1000.0);

        validator.trust_gravity =
            validator
                .trust_gravity
                .clamp(0.0, 1000.0);
    }
}