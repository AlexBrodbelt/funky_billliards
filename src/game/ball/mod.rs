use bevy::prelude::*;

pub mod components;
mod systems;
mod resources;

use systems::*;

use crate::{AppState, systems::despawn};

use self::components::Ball;

use super::GameSetUpState;
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CueBallState>()
            // On Enter GameSetup::CueBallSetup Systems
            .add_systems(
                OnEnter(GameSetUpState::BallSetUp),
                (
                    spawn_balls
                )
                    .run_if(in_state(AppState::GameSetUp))
            )
            .add_systems(
                OnEnter(GameSetUpState::CueBallSetUp),
                (
                    spawn_cue_ball.run_if(in_state(AppState::GameSetUp)),
                    tell_me_why_no_spawn_cue_ball
                )     
            )
            // Update systems
            .add_systems(
                Update,
                set_cue_ball.run_if(in_state(GameSetUpState::CueBallSetUp)))            
            // On Exit AppState::Game Systems
            .add_systems(
                OnEnter(AppState::Menu),
                despawn::<Ball>,
            );
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CueBallState {
    InPlay,
    #[default]
    InHand,
}