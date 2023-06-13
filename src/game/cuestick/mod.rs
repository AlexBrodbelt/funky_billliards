use bevy::prelude::*;

use crate::AppState;

use self::{systems::*, resources::CueStickLifetimeTimer};

use super::{GameSetupState, GameState};

mod components;
pub mod resources;
pub mod systems;


pub struct CueStickPlugin;

impl Plugin for CueStickPlugin {
    fn  build(&self, app: &mut App) {
        app.add_system(
                spawn_cue_stick
                    .in_schedule(OnEnter(GameSetupState::ShotSetup))
            )
            .add_systems(
                (
                    set_cue_stick,
                    compute_strike_force,
                    strike_cue_ball
                )
                    .in_set(OnUpdate(GameSetupState::ShotSetup))
                    .in_set(OnUpdate(AppState::GameSetup))
            )
            .add_system(insert_cue_stick_lifetime_timer.in_schedule(OnExit(AppState::GameSetup)))
            .add_systems(
                (
                    cue_stick_cooldown,
                    despawn_cue_stick
                )
                    .in_set(OnUpdate(GameState::ShotCooldown))
                    .in_set(OnUpdate(AppState::Game))
                    .distributive_run_if(resource_exists::<CueStickLifetimeTimer>())
                )
            .add_system(remove_cue_stick_lifetime_timer.in_schedule(OnExit(GameState::ShotCooldown)));                  
    }
}