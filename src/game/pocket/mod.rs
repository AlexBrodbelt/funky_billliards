use bevy::prelude::*;

use self::systems::spawn_pockets;

pub mod components;
mod systems;

use systems::*;

pub struct PocketPlugin;

impl Plugin for PocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pockets)
            .add_system(pocket_condition);
    }
}