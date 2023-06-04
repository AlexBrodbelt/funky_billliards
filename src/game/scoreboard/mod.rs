use bevy::prelude::*;

pub mod resources;
mod systems;

use systems::*;
use resources::*;

pub struct ScoreBoardPlugin;

impl Plugin for ScoreBoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .add_startup_system(spawn_scoreboard)
            .add_system(update_scoreboard);
    }
}