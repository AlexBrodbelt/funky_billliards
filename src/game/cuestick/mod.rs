use bevy::prelude::*;

use crate::AppState;

use self::systems::spawn_cuestick;

use super::ball::CueBallState;

pub mod components;
pub mod systems;

use self::systems::*;


pub struct CueStickPlugin;

impl Plugin for CueStickPlugin {
    fn  build(&self, app: &mut App) {
        app.add_system(
                spawn_cuestick
                    .in_schedule(OnEnter(CueBallState::InPlay))
                    .in_set(OnUpdate(AppState::Game))
            )
            .add_system(
                set_cuestick
                    .in_set(OnUpdate(CueBallState::InPlay))
                    .in_set(OnUpdate(AppState::Game))
            );                  
    }
}