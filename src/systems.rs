use bevy::{prelude::*, app::AppExit};
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::{AppState, game::{SimulationState, ball::CueBallState}};

/// Handles transitions between AppState, SimulationState, CueBallState states.
pub fn state_transitions(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    cue_ball_state: Res<State<CueBallState>>,
) {
    // Press G: _ -> AppState::GameSetup and CueBallState::InHand
    if keyboard_input.just_pressed(KeyCode::G)
        && app_state.0 != AppState::GameSetup 
        {
        commands.insert_resource(NextState(Some(AppState::GameSetup)));
        commands.insert_resource(NextState(Some(CueBallState::InHand)));
    }
    // Press M: _ -> AppState::Menu
    if keyboard_input.just_pressed(KeyCode::M)
        && app_state.0 != AppState::Menu
        {
        commands.insert_resource(NextState(Some(AppState::Menu)));
    }
    // Press Return/Enter: AppState::GameSetup -> AppState::Game and SimulationState::Paused
    if keyboard_input.just_pressed( KeyCode::Return) 
        && cue_ball_state.0 == CueBallState::InPlay
        && app_state.0 == AppState::GameSetup
        {
        commands.insert_resource(NextState(Some(AppState::Game)));
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
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
    cue_ball_state: Res<State<CueBallState>>,  
) {
    if simulation_state.is_changed() {
        println!("{:?}", simulation_state.0);
    }

    if cue_ball_state.is_changed() {
        println!("{:?}", cue_ball_state.0);
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
