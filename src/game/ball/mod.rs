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
            .add_system(
                spawn_balls
                    .in_schedule(OnEnter(AppState::GameSetup))
                    .run_if(in_state(GameSetUpState::BallSetup))
            )
            .add_system(
                spawn_cue_ball
                    .in_schedule(OnEnter(GameSetUpState::CueBallSetup))
                    .run_if(in_state(AppState::GameSetup))
            )
            // OnUpdate GameSetup::CueBallSetup Systems
            .add_system(set_cue_ball.in_set(OnUpdate(GameSetUpState::CueBallSetup)))            
            // On Exit AppState::Game Systems
            .add_system(despawn_balls.in_schedule(OnEnter(AppState::Menu)));
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CueBallState {
    InPlay,
    #[default]
    InHand,
}