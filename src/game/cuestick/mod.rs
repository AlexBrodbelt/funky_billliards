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
            .add_system(
                spawn_cue_stick
                    .in_schedule(OnEnter(GameSetUpState::ShotSetup))
            )
            .add_systems(
                (
                    set_cue_stick,
                    compute_wind_up_distance,
                    strike_cue_ball,
                )
                    .in_set(OnUpdate(GameSetUpState::ShotSetup))
                    .in_set(OnUpdate(AppState::GameSetup))
                    .in_set(OnUpdate(SimulationState::Running))
            )
            .add_systems(
                (
                    handle_cue_stick_motion,
                )
                    .in_set(OnUpdate(GameState::ShotCooldown))
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running))
                );
    }
}