use bevy::prelude::*;

pub mod resources;
mod styles;
mod systems;
mod components;

use systems::*;
use resources::*;

use crate::AppState;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Game),
                (
                spawn_scoreboard
                    .run_if(not(resource_exists::<Scoreboard>()))
                )      
            )
            .add_systems(
                Update,
                (
                update_scoreboard.run_if(in_state(AppState::Game))
                )
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                despawn_scoreboard
                    .run_if(resource_exists::<Scoreboard>())
                )
            );
    }
}