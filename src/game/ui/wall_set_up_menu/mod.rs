pub mod systems;
mod styles;
pub mod components;

use bevy::prelude::*;

use crate::game::GameSetUpState;

use self::systems::{
    interactions::*,
    layout::*,
};

pub struct WallSetUpMenuPlugin;

impl Plugin for WallSetUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_wall_set_up_menu.in_schedule(OnEnter(GameSetUpState::WallSetup)))
            .add_system(interact_with_button.in_set(OnUpdate(GameSetUpState::PocketSetup)))
            .add_system(despawn_wall_set_up_menu.in_schedule(OnExit(GameSetUpState::PocketSetup)));
    }
}