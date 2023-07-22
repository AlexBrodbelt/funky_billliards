use bevy::prelude::*;

pub mod components;
mod systems;
mod resources;

use systems::*;

use crate::AppState;

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
                    spawn_cue_ball
                )
                    .run_if(in_state(AppState::GameSetUp))
            )
            // Update systems
            .add_systems(
                Update,
                set_cue_ball.run_if(in_state(GameSetUpState::CueBallSetUp)))            
            // On Exit AppState::Game Systems
            .add_systems(
                OnEnter(AppState::Menu),
                despawn_balls
            );
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CueBallState {
    InPlay,
    #[default]
    InHand,
}