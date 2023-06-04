use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;


pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls);
    }
}

