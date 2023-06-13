use bevy::window::PrimaryWindow;
use bevy::{prelude::*, app::AppExit};
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::game::{GameSetupState, GameState};
use crate::resources::*;
use crate::{AppState, game::{SimulationState, ball::CueBallState}};

pub fn get_cursor_position(
    mut cursor_moved: EventReader<CursorMoved>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position_resource: ResMut<CursorPosition>,
) {
    let Ok(primary) = primary_window_query.get_single() else {
        return;
    };
    let mut cursor_position = match cursor_moved.iter().last() {
        Some(cursor) => cursor.position,
        None => return,
    };

    cursor_position.x -= 0.5 * primary.width();
    cursor_position.y -= 0.5 * primary.height();

    cursor_position_resource.0 = cursor_position;
}


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
        commands.insert_resource(NextState(Some(GameSetupState::CueBallSetup)));
        commands.insert_resource(NextState(Some(CueBallState::InHand)));
    }
    // Press M: _ -> AppState::Menu
    if keyboard_input.just_pressed(KeyCode::M)
        && app_state.0 != AppState::Menu
        {
        commands.insert_resource(NextState(Some(AppState::Menu)));
    }
    // Press Return/Enter: GameSetupState::CueBallSetup -> GameSetupState::ShotSetup
    if keyboard_input.just_pressed(KeyCode::Return) 
        && cue_ball_state.0 == CueBallState::InPlay
        && app_state.0 == AppState::GameSetup
        {
        // commands.insert_resource(NextState(Some(AppState::GameSetup)));
        commands.insert_resource(NextState(Some(GameSetupState::ShotSetup)));
        // commands.insert_resource(NextState(Some(SimulationState::Paused)));
    } 
}

pub fn toggle_physics_pipeline(
    app_state: Res<State<AppState>>,
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
    game_setup_state: Res<State<GameSetupState>>,
    game_state: Res<State<GameState>>,
    cue_ball_state: Res<State<CueBallState>>,  
    simulation_state: Res<State<SimulationState>>, 
) {
    if simulation_state.is_changed() {
        println!("{:?}", simulation_state.0);
    }

    if cue_ball_state.is_changed() {
        println!("{:?}", cue_ball_state.0);
    }

    if game_setup_state.is_changed() {
        println!("{:?}", game_setup_state.0);
    }

    if game_state.is_changed() {
        println!("{:?}", game_state.0);
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
