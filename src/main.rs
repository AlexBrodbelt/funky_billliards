// #![warn(clippy::all)]

use bevy::{
    prelude::*,
    // sprite::collide_aabb::{collide, Collision},
    // sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

pub mod components;
pub mod config;
mod events;
pub mod resources;
mod systems;

// use components::*;
use config::*;
// use events::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1500.0)) // needs to be tweaked
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource( RapierConfiguration {
            gravity : Vec2::ZERO,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system(play_collision_sound)
        .add_system(pocket_condition)
        // .add_system(display_events)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}


















