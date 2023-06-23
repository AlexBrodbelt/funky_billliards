use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{AppState, config::STOPPING_THRESHOLD};

use super::{resources::*, SimulationState, ball::{components::{Ball, CueBall}}, GameSetupState};


pub fn spawn_camera(
    mut commands: Commands,    
) {
    commands.spawn( Camera2dBundle::default());
}

pub fn spawn_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Sound
    let ball_collision_sound = asset_server.load("sounds\\breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));
}


pub fn play_collision_sound(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.r
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

/// Pauses/Resumes the Game simulation
pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                next_simulation_state.set(SimulationState::Paused);
            }
            SimulationState::Paused => {
                next_simulation_state.set(SimulationState::Running);
            }
        }
    }
}

/// Sets the velocity of a ball to zero whenever they are below the [`STOPPING_THRESHOLD`]
pub fn stopping_threshold(
    mut ball_query: Query<&mut Velocity, With<Ball>>,
) {
    for mut ball_velocity in &mut ball_query {
        if ball_velocity.linvel.length_squared() < STOPPING_THRESHOLD {
            *ball_velocity = Velocity::zero();
        }
    }
}

/// detects if sprites are not moving
fn balls_not_moving(
    ball_query: Query<Entity, (With<Ball>, Changed<Transform>)>,
) -> bool {
    ball_query.is_empty() 
}

/// Whenever the balls are not moving anymore it handles the transition from the current player to the next.
/// Moreover, it detects whether the cue ball has been pocketed and sets [`GameSetupState`] to the appropriate state
pub fn switch_player_condition(
    ball_query: Query<Entity, (With<Ball>, Changed<Transform>)>, 
    cue_ball_query: Query<&Transform, With<CueBall>>,
    mut active_player: ResMut<ActivePlayer>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetupState>>,
    mut table_status: ResMut<TableStatus>,
) {
    if balls_not_moving(ball_query) {
        active_player.switch_player();
        println!("{:?}", active_player.0);
        next_app_state.set(AppState::GameSetup);
        // If the cue ball is still on the table set the GameSetupState to ShotSetup otherwise set to CueBallSetup
        if cue_ball_query.is_empty() {
            next_game_setup_state.set(GameSetupState::CueBallSetup);
        } else {
            let cue_ball_transform = cue_ball_query.single();
            table_status.cue_ball_status.initial_position = Some(cue_ball_transform.translation.truncate());
            next_game_setup_state.set(GameSetupState::ShotSetup);
        } 
    }
}




