use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_rapier2d::prelude::*;

use super::{resources::*, SimulationState};


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


pub fn get_cursor_position(
    mut cursor: EventReader<CursorMoved>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>
) -> Option<Vec2> {
    let Ok(primary) = primary_window_query.get_single() else {
        return None;
    };
    let mut cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return None
    };
    cursor_position.x -= 0.5 * primary.width();
    cursor_position.y -= 0.5 * primary.height();
    Some(cursor_position)

}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
            } 
        }
    }
}


