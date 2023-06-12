use bevy::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn cueball_setup_to_shot_setup() {
        let mut app = App::new();

        app
            .add_state::<AppState>()
            .add_state::<GameSetupState>()
            .add_state::<SimulationState>()
            .add_state::<CueBallState>()
            .add_system(state_transitions);

        let mut input = Input::<KeyCode>::default();
        input.press(KeyCode::Space);
    }
}