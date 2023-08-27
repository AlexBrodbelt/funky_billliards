// #![warn(clippy::all)]
mod game;
mod main_menu;
mod config;
pub mod resources;
mod systems;
pub mod styles;

use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use renet::{
    transport::ClientAuthentication, ConnectionConfig, NETCODE_USER_DATA_BYTES_ERROR
};
use bevy_xpbd_2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use std::{net::UdpSocket, time::SystemTime};

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
        .add_state::<AppState>()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Renet Plugin
        .add_plugins(RenetClientPlugin)
        // Bevy Inspector EGUI
        .add_plugins(WorldInspectorPlugin::new())
        // Bevy XPBD Plugins
        .add_plugins(PhysicsPlugins::default()) 
        // Bevy Prototype Lyon
        .add_plugins(ShapePlugin)
        // My Plugins
        .add_plugins(
            (
                GamePlugins,
                MainMenuPlugin,
            )
        )
        // Bevy XPBD Resources
        .insert_resource(Gravity(Vec2::ZERO))
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
                toggle_physics_loop,
                exit_app
            )
        )
        .run();
    }