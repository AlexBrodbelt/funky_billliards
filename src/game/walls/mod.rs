use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;

use systems::*;



pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_walls.in_schedule(OnEnter(AppState::GameSetup)))
            .add_system(despawn_walls.in_schedule(OnEnter(AppState::Menu)));
    }
}

