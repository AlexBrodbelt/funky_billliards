use bevy::prelude::*;

pub mod components;
pub mod systems;

use crate::AppState;
use super::GameSetUpState;

use systems::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]

pub enum WallSetUpState {
    Edit,
    #[default]
    Disabled
}

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<WallSetUpState>()
            .add_systems(
                Update,
                (
                    wall_set_up_events_handler
                )
                .run_if(in_state(GameSetUpState::WallSetUp))
                .run_if(in_state(AppState::GameSetUp))
                // .run_if(in_state(WallSetUpState::Edit))
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                    despawn_wall,
                )
            );
    }
}

