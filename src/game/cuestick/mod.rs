use bevy::prelude::*;

use crate::AppState;

use self::{
    systems::*,
    resources::StrikeForce
};

use super::{ball::CueBallState, GameSetupState};

mod components;
pub mod resources;
pub mod systems;


pub struct CueStickPlugin;

impl Plugin for CueStickPlugin {
    fn  build(&self, app: &mut App) {
        app.add_system(
                spawn_cuestick
                    .in_schedule(OnEnter(GameSetupState::ShotSetup))
                    // .in_schedule(OnEnter(GameSetupState::ShotSetup))
            )
            .add_systems(
                (
                    set_cuestick,
                    strike_cueball
                )
                    .in_set(OnUpdate(GameSetupState::ShotSetup))
            )
            .add_system(
                despawn_cuestick
                    .in_schedule(OnExit(GameSetupState::ShotSetup))
            );                  
    }
}