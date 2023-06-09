use bevy::prelude::*;

use crate::AppState;

use self::{
    systems::*,
    resources::{MouseWheelActive, StrikeImpulse}};

use super::{ball::CueBallState, GameSetupState};

mod components;
pub mod resources;
pub mod systems;


pub struct CueStickPlugin;

impl Plugin for CueStickPlugin {
    fn  build(&self, app: &mut App) {
        app.add_system(
                spawn_cuestick
                    // .in_schedule(OnEnter(CueBallState::InPlay))
                    .in_schedule(OnEnter(GameSetupState::ShotSetup))
            )
            .add_system(
                set_cuestick
                    .in_set(OnUpdate(GameSetupState::ShotSetup))
            )
            .add_system(
                strike_cueball
                    .in_set(OnUpdate(GameSetupState::ShotSetup))
            )
            .add_system(
                despawn_cuestick
                    .in_schedule(OnEnter(AppState::Game))
            );                  
    }
}