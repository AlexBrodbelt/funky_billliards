// #![warn(clippy::all)]
mod game;
mod main_menu;
mod config;
pub mod resources;
mod systems;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;


use game::{
    GamePlugins, 
    systems::*,
};

use main_menu::MainMenuPlugin;
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
        .add_plugin(WorldInspectorPlugin::new())
        // Bevy Rapier Plugins
        .add_plugin(RapierDebugRenderPlugin::default()) 
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1500.0)) // needs to be tweaked
        // Bevy Rapier Egui Plugin
        // .add_plugin(InspectableRapierPlugin)
        // My Plugins
        .add_plugins(GamePlugins)
        .add_plugin(MainMenuPlugin)

        // Register App States
        

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
                exit_app
            )
        )
        .run();
    }