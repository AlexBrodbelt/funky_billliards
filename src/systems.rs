use bevy::window::PrimaryWindow;
use bevy::{prelude::*, app::AppExit};
use bevy_rapier2d::prelude::RapierConfiguration;

use crate::game::{GameSetUpState, GameState};
use crate::resources::*;
use crate::{AppState, game::{SimulationState, ball::CueBallState}};

/// Gets the cursors current position and updates the [`CursorPosition`] Resource
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


/// Handles transitions between [`AppState`], [`SimulationState`], [`CueBallState`] states.
pub fn state_transitions(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    cue_ball_state: Res<State<CueBallState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
    mut next_cue_ball_state: ResMut<NextState<CueBallState>>,
) {
    // Press G: _ -> AppState::GameSetup and CueBallState::InHand
    if keyboard_input.just_pressed(KeyCode::G)
        && app_state.0 != AppState::GameSetup 
        {
        next_app_state.set(AppState::GameSetup);
        next_game_setup_state.set(GameSetUpState::CueBallSetup);
        next_cue_ball_state.set(CueBallState::InHand);
    }
    // Press M: _ -> AppState::Menu
    if keyboard_input.just_pressed(KeyCode::M)
        && app_state.0 != AppState::Menu
        {
        next_app_state.set(AppState::Menu);
    }
    // Press Return/Enter: GameSetupState::CueBallSetup -> GameSetupState::ShotSetup
    if keyboard_input.just_pressed(KeyCode::Return) 
        && cue_ball_state.0 == CueBallState::InPlay
        && app_state.0 == AppState::GameSetup
        {
        next_game_setup_state.set(GameSetUpState::ShotSetup);
    } 
}

/// pauses/resumes the physics pipeline in [`RapierConfiguration`] depending if in the appropriate app state.
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

/// This macro expands into if statement that checks if the state has changed
/// and if so prints the current state.
macro_rules! print_if_state_changed {
    ( $( $state:ident ),* ) => {
        {
            $(
            if $state.is_changed() {
            println!("{:?}", $state.0);
            }
            )*
        }
    };
}

/// Prints if any state has changed.
pub fn display_state(
    app_state: Res<State<AppState>>,
    game_setup_state: Res<State<GameSetUpState>>,
    game_state: Res<State<GameState>>,
    cue_ball_state: Res<State<CueBallState>>,  
    simulation_state: Res<State<SimulationState>>, 
) {
    print_if_state_changed!(
        app_state,
        game_setup_state,
        game_state,
        cue_ball_state,
        simulation_state
    );
}

/// Exit the app by pressing Escape.
pub fn exit_app(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}
