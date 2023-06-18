use bevy::prelude::*;
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


