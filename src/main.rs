// #![warn(clippy::all)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod game;
mod menu;
mod config;
pub mod resources;
mod systems;


use game::{
    GamePlugins, 
    systems::*,
};

use menu::MenuPlugin;
use resources::CursorPosition;
use systems::*;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    GameSetup,
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
        .add_plugin(MenuPlugin)
        // Bevy Rapier Resources
        .insert_resource( RapierConfiguration {
            gravity : Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(CursorPosition::default())
        // Startup Systems
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_sound)
        // Systems
        .add_systems(
            (   
                get_cursor_position,
                display_state,
                state_transitions,
                toggle_physics_pipeline,
                exit_game
            )
        )
        // .add_system(bevy::window::close_on_esc)
        .run();
    }