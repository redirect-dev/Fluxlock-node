use fluxlock_core::types::{
    Validator,
    ContinuityState,
};

pub fn update_authority_legitimacy(
    validator: &mut Validator,
) {
    let mut legitimacy = 100.0;

    legitimacy -=
        validator.continuity_suspicion * 0.50;

    legitimacy -=
        validator.fracture_severity * 0.25;

    legitimacy -=
        validator.quarantine_level * 0.25;

    match validator.continuity_state {

        ContinuityState::Healthy => {}

        ContinuityState::Evolving => {
            legitimacy -= 5.0;
        }

        ContinuityState::Recovering => {
            legitimacy -= 25.0;
        }

        ContinuityState::Rehabilitating => {
            legitimacy -= 15.0;
        }

        ContinuityState::Quarantined => {
            legitimacy -= 50.0;
        }

        ContinuityState::Fractured => {
            legitimacy -= 75.0;
        }

        ContinuityState::Exiled => {
            legitimacy = 0.0;
        }
    }

    validator.authority_legitimacy =
        legitimacy.clamp(0.0, 100.0);

    validator.effective_authority =
        validator.authority_points
        *
        (
            validator.authority_legitimacy
            / 100.0
        );
}