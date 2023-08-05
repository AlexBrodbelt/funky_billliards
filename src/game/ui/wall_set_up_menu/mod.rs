pub mod components;
pub mod events;
pub mod systems;
mod styles;

use bevy::prelude::*;

use crate::game::GameSetUpState;

use self::{systems::{
    interactions::*,
    layout::*,
}, events::WallSetUpEvent};



pub struct WallSetUpMenuPlugin;

impl Plugin for WallSetUpMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WallSetUpEvent>()
            .add_systems(
                OnEnter(GameSetUpState::WallSetUp),
                (
                spawn_wall_set_up_menu,
                )
            )
            .add_systems(
                Update,
                (
                interact_with_button.run_if(in_state(GameSetUpState::WallSetUp)),
                )
            )
            .add_systems(
                OnExit(GameSetUpState::WallSetUp),
                (
                despawn_wall_set_up_menu,
                )
            );
    }
}