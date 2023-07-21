use bevy::prelude::*;

use crate::AppState;

use self::systems::spawn_pockets;

pub mod components;
mod systems;

use systems::*;

use super::GameSetUpState;

pub struct PocketPlugin;

impl Plugin for PocketPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_pockets.in_schedule(OnEnter(GameSetUpState::PocketSetUp)))
            .add_system(pocket_condition.in_set(OnUpdate(AppState::Game)))
            .add_system(despawn_pockets.in_schedule(OnEnter(AppState::Menu)));
    }
}