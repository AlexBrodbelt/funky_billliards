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
                (
                    set_wall_vertex,
                )
                .in_set(OnUpdate(GameSetUpState::WallSetUp))
                .in_set(OnUpdate(AppState::GameSetup))
                .in_set(OnUpdate(WallSetUpState::Edit))
            )
            .add_system(despawn_wall.in_schedule(OnEnter(AppState::Menu)));
    }
}

