use fluxlock_core::types::{
    Validator,
};

// =========================
// ☠ FRACTURE WAVE
// =========================
pub fn propagate_fracture_wave(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    for validator in
        validators.iter_mut()
    {

        let mut fracture_pressure = 0.0;

        for peer in snapshot.iter()
        {

            if peer.id == validator.id {

                continue;
            }

            if !peer.chain_valid {

                fracture_pressure +=
                    peer.fracture_severity
                    * 0.02;

                fracture_pressure +=
                    peer.entropy_output
                    * 0.01;
            }
        }

        validator.drift +=
            fracture_pressure;

        validator.consensus_pressure +=
            fracture_pressure * 0.5;

        validator.instability_radius +=
            fracture_pressure * 0.25;

        validator.drift =
            validator
                .drift
                .clamp(0.0, 1000.0);
    }
}