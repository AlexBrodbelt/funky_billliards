pub mod systems;
mod styles;
pub mod components;

use bevy::prelude::*;

use crate::game::GameSetUpState;

use self::systems::{
    interactions::*,
    layout::*,
};

pub struct PocketSetUpMenuPlugin;

impl Plugin for PocketSetUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_pocket_set_up_menu.in_schedule(OnEnter(GameSetUpState::PocketSetUp)))
            .add_system(interact_with_button.in_set(OnUpdate(GameSetUpState::PocketSetUp)))
            .add_system(despawn_pocket_set_up_menu.in_schedule(OnExit(GameSetUpState::PocketSetUp)));
    }
}
