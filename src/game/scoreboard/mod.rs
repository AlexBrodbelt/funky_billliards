use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;
use resources::*;

use crate::AppState;

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
                spawn_scoreboard
                    .run_if(not(resource_exists::<Scoreboard>()))
                    .in_schedule(OnEnter(AppState::Game))
            )
            .add_system(update_scoreboard.in_set(OnUpdate(AppState::Game)))
            .add_system(
                despawn_scoreboard
                    .run_if(resource_exists::<Scoreboard>())
                    .in_schedule(OnExit(AppState::Game))
            );
    }
}