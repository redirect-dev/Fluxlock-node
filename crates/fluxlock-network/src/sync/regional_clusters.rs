use fluxlock_core::types::{
    Validator,
};

// =========================
// 🌐 REGIONAL CLUSTERING
// =========================
pub fn evolve_regional_clusters(
    validators: &mut Vec<Validator>,
) {

    let validator_snapshot =
        validators.clone();

    for validator in
        validators.iter_mut()
    {

        // =========================
        // 🌐 LOCAL REGION
        // =========================
        let local_cluster =
            validator.topology_cluster;

        let neighbors:
            Vec<&Validator> =
            validator_snapshot
                .iter()
                .filter(|v|
                    v.topology_cluster
                        == local_cluster
                    &&
                    v.id != validator.id
                )
                .collect();

        if neighbors.is_empty() {

            continue;
        }

        // =========================
        // 🧠 REGIONAL METRICS
        // =========================
        let average_trust: f64 =
            neighbors
                .iter()
                .map(|v| v.trust)
                .sum::<f64>()
                / neighbors.len() as f64;

        let average_stability: f64 =
            neighbors
                .iter()
                .map(|v|
                    v.lineage_stability
                )
                .sum::<f64>()
                / neighbors.len() as f64;

        let average_pressure: f64 =
            neighbors
                .iter()
                .map(|v|
                    v.consensus_pressure
                )
                .sum::<f64>()
                / neighbors.len() as f64;

        let average_healing: f64 =
            neighbors
                .iter()
                .map(|v|
                    v.healing_wave
                )
                .sum::<f64>()
                / neighbors.len() as f64;

        // =========================
        // 🌊 CLUSTER INFLUENCE
        // =========================
        validator.trust +=
            average_trust
            * 0.0008;

        validator.lineage_stability +=
            average_stability
            * 0.0005;

        validator.consensus_pressure +=
            average_pressure
            * 0.0003;

        validator.healing_wave +=
            average_healing
            * 0.0005;

        // =========================
        // 🛡 HEALTHY CLUSTER BONUS
        // =========================
        if average_trust > 90.0 {

            validator.stabilization_power +=
                0.05;

            validator.immune_response +=
                0.02;
        }

        // =========================
        // ☠ FRACTURED REGION
        // =========================
        if average_pressure > 25.0 {

            validator.instability_radius +=
                0.15;

            validator.entropy_output +=
                0.05;
        }

        // =========================
        // 🌐 CONTINUITY GRAVITY
        // =========================
        validator.trust_gravity +=
            (
                validator.trust
                * 0.0005
            );

        validator.resonance_score +=
            (
                validator.healing_wave
                * 0.0002
            );

        // =========================
        // 🔒 LIMITS
        // =========================
        validator.trust =
            validator
                .trust
                .clamp(0.0, 100.0);

        validator.lineage_stability =
            validator
                .lineage_stability
                .clamp(0.0, 100.0);

        validator.consensus_pressure =
            validator
                .consensus_pressure
                .clamp(0.0, 1000.0);

        validator.healing_wave =
            validator
                .healing_wave
                .clamp(0.0, 1000.0);

        validator.trust_gravity =
            validator
                .trust_gravity
                .clamp(0.0, 1000.0);

        validator.resonance_score =
            validator
                .resonance_score
                .clamp(0.0, 1000.0);
    }
}