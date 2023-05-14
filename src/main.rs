// #![warn(clippy::all)]

use bevy::{
    prelude::*,
    // sprite::collide_aabb::{collide, Collision},
    // sprite::MaterialMesh2dBundle,
};

pub mod components;
pub mod config;
mod events;
pub mod resources;
mod systems;

use components::*;
use config::*;
use events::*;
use resources::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        // Add our gameplay simulation systems to the fixed timestep schedule
        .add_systems(
            (
                check_for_collisions,
                move_system.before(check_for_collisions),
                // move_paddle
                //     .before(check_for_collisions)
                //     .after(apply_velocity),
                velocity_system.before(move_system),
                play_collision_sound.after(check_for_collisions),
            )
                .in_schedule(CoreSchedule::FixedUpdate),
        )
        // Configure how frequently our gameplay systems are run
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_system(update_scoreboard)
        .add_system(bevy::window::close_on_esc)
        .run();
}


















