use fluxlock_core::types::{
    Validator,
};

// =========================
// 🛡 STABILIZATION FIELD
// =========================
pub fn apply_stabilization_field(
    validators: &mut Vec<Validator>,
) {

    let snapshot =
        validators.clone();

    for validator in
        validators.iter_mut()
    {

        let mut stabilization = 0.0;

        for peer in snapshot.iter()
        {

            if peer.id == validator.id {

                continue;
            }

            if peer.chain_valid {

                stabilization +=
                    peer.continuity_anchor_strength
                    * 0.003;

                stabilization +=
                    peer.healing_wave
                    * 0.002;
            }
        }

        validator.drift -=
            stabilization;

        validator.fracture_severity -=
            stabilization * 0.25;

        validator.trust +=
            stabilization * 0.05;

        validator.drift =
            validator.drift.max(0.0);

        validator.fracture_severity =
            validator
                .fracture_severity
                .max(0.0);
    }
}