use bevy::prelude::*;

pub mod components;
mod systems;
mod resources;

use systems::*;

use crate::AppState;


pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_state::<CueBallState>()
            // On Enter AppState::Game Systems
            .add_system(spawn_balls.in_schedule(OnEnter(AppState::Game)))
            // On Enter AppState::Game Systems
            .add_system(despawn_balls.in_schedule(OnExit(AppState::Game)))
            // OnUpdate AppState::GameSetup Systems
            .add_system(set_cue_ball.in_set(OnUpdate(AppState::GameSetup)))
            // OnUpdate CueBallState::InPlay Systems
            .add_system(spawn_cue_ball.in_schedule(OnEnter(CueBallState::InPlay)));
    }
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CueBallState {
    InPlay,
    #[default]
    InHand,
}