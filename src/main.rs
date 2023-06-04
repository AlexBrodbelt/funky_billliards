// #![warn(clippy::all)]

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


mod config;
mod game;
mod game_menu;

use config::*;
use game::{
    GamePlugins, 
    systems::*
};


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugins)
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_system(display_events)

        .add_state::<AppState>()
        
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1500.0)) // needs to be tweaked
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource( RapierConfiguration {
            gravity : Vec2::ZERO,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_sound)
        .run();
    }