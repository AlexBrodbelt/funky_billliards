use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

use crate::AppState;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        // On Enter Systems
        app.add_system(spawn_balls.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_balls.in_schedule(OnExit(AppState::Game)));
    }
}