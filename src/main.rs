// #![warn(clippy::all)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


mod config;
mod systems;
mod game;
mod menu;

use game::{
    GamePlugins, 
    systems::*
};

use systems::*;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // Bevy Rapier Plugins
        .add_plugin(RapierDebugRenderPlugin::default()) // Debugger Plugin
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1500.0)) // needs to be tweaked
        // My Plugins
        .add_plugins(GamePlugins)
        // Bevy Rapier Resources
        .insert_resource( RapierConfiguration {
            gravity : Vec2::ZERO,
            ..Default::default()
        })
        // Startup Systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_sound)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_menu_state)
        .run();
    }