use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

use super::{GameSetUpState, GameState, SimulationState};

mod components;
pub mod resources;
pub mod systems;


pub struct CueStickPlugin;

impl Plugin for CueStickPlugin {
    fn  build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameSetUpState::ShotSetUp),
                (
                    spawn_cue_stick,
                )
            )
            .add_systems(
                Update,
                (
                    set_cue_stick,
                    compute_wind_up_distance,
                    strike_cue_ball,
                )
                    .run_if(in_state(GameSetUpState::ShotSetUp))
                    .run_if(in_state(AppState::GameSetUp))
                    .run_if(in_state(SimulationState::Running))
            )
            .add_systems(
                Update,
                (
                    handle_cue_stick_motion,
                )
                    .run_if(in_state(GameState::ShotCoolDown))
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running))
                );
    }
}