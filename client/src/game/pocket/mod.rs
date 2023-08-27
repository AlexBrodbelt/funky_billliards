use bevy::prelude::*;

use crate::{AppState, systems::despawn};

use self::{systems::spawn_pocket, components::Pocket};

pub mod components;
mod systems;

use systems::*;

use super::GameSetUpState;

pub struct PocketPlugin;

impl Plugin for PocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pocket_set_up_event_handler,
                )
                .run_if(in_state(GameSetUpState::PocketSetUp))
                .run_if(in_state(AppState::GameSetUp))
            )
            .add_systems(
                Update,
                (
                    pocket_condition.run_if(in_state(AppState::Game)),
                )
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                despawn::<Pocket>,
                )
            );
    }
}