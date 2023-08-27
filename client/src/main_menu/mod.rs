use bevy::prelude::*;

pub mod systems;
mod styles;
mod components;

use systems::layout::*;
use systems::interactions::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_systems(
                OnEnter(AppState::Menu),
                (
                spawn_main_menu,
                )
            )
            // OnUpdate Systems
            .add_systems(
                Update,
                (
                    interact_with_button.run_if(in_state(AppState::Menu)),
                )
            )
            // OnExit Systems
            .add_systems(
                OnExit(AppState::Menu),
                (
                    despawn_main_menu,
                )
            );
    }
}




