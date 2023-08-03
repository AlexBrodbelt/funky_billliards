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
            .add_systems(
                OnEnter(GameSetUpState::PocketSetUp),
                spawn_pocket_set_up_menu
            )
            .add_systems(
                Update,
                (
                interact_with_button.run_if(in_state(GameSetUpState::PocketSetUp)),
                )
            )
            .add_systems(
                OnExit(GameSetUpState::PocketSetUp),
                (
                despawn_pocket_set_up_menu,
                )
            );
    }
}
