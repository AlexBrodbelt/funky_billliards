// #![warn(clippy::all)]
mod game;
mod main_menu;
mod config;
pub mod resources;
mod systems;
pub mod styles;

use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;


use game::{
    GamePlugins, 
    systems::*,
};

use main_menu::MainMenuPlugin;
use resources::{CursorPosition, PeriodicTimer};
use systems::*;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    GameSetUp,
    Game,
    GameOver,
}



fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(WorldInspectorPlugin::new())
        // Bevy Rapier Plugins
        .add_plugins(
            (
                RapierDebugRenderPlugin::default(),
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1500.0)
            )
        ) 
        // Bevy Prototype Lyon
        .add_plugins(ShapePlugin)
        // Bevy Rapier Egui Plugin
        // .add_plugins(InspectableRapierPlugin)
        // My Plugins
        .add_plugins(
            (
                GamePlugins,
                MainMenuPlugin,
            )
        )
        // Bevy Rapier Resources
        .insert_resource( RapierConfiguration {
            gravity : Vec2::ZERO,
            ..Default::default()
        })
        .insert_resource(CursorPosition::default())
        // Debugging resource
        // .insert_resource(PeriodicTimer(Timer::new( Duration::from_secs(5), TimerMode::Repeating)))
        // Startup Systems
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_sound
            )
        )
        // Systems
        .add_systems(
            Update,
            (   
                get_cursor_position,
                display_state,
                // debug_states,
                state_transitions,
                toggle_physics_pipeline,
                exit_app
            )
        )
        .run();
    }