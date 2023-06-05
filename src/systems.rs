use bevy::{prelude::*, app::AppExit};
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::{AppState, game::SimulationState};

pub fn state_transitions(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.0 != AppState::Game {
        commands.insert_resource(NextState(Some(AppState::Game)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
    }

    if keyboard_input.just_pressed(KeyCode::M) && app_state.0 != AppState::Menu {
        commands.insert_resource(NextState(Some(AppState::Menu)));
    }  
}

pub fn toggle_physics_pipeline(
    app_state: Res<State<AppState>> ,
    simulation_state: Res<State<SimulationState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    if app_state.is_changed() || simulation_state.is_changed() {
        if app_state.0 != AppState::Game || simulation_state.0 != SimulationState::Running {
            rapier_config.physics_pipeline_active = false;
        } else {
            rapier_config.physics_pipeline_active = true;
        }
    }

}

pub fn display_state(
    app_state: Res<State<AppState>> ,
    simulation_state: Res<State<SimulationState>>,   
) {
    if simulation_state.is_changed() {
        println!("{:?}", simulation_state.0)
    }

    if app_state.is_changed() {
        println!("{:?}", app_state.0);
    }  
}


pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
