use fluxlock_core::types::{
    Validator,
};

// =========================
// 🌐 TOPOLOGY PRESSURE
// =========================
pub fn apply_topology_pressure(
    validators: &mut Vec<Validator>,
) {

    let total_pressure: f64 =
        validators
            .iter()
            .map(|v|
                v.consensus_pressure
            )
            .sum();

    let average_pressure =
        total_pressure
        / validators.len()
            .max(1) as f64;

    for validator in
        validators.iter_mut()
    {

        validator.regional_pressure +=
            average_pressure
            * 0.015;

        validator.resonance_score +=
            validator.trust_gravity
            * 0.002;

        validator.healing_wave +=
            validator.immune_strength
            * 0.001;

        validator.instability_radius +=
            validator.entropy_output
            * 0.002;

        validator.regional_pressure =
            validator
                .regional_pressure
                .clamp(0.0, 100.0);

        validator.resonance_score =
            validator
                .resonance_score
                .clamp(0.0, 1000.0);

        validator.healing_wave =
            validator
                .healing_wave
                .clamp(0.0, 1000.0);

        validator.instability_radius =
            validator
                .instability_radius
                .clamp(0.0, 1000.0);
    }
}