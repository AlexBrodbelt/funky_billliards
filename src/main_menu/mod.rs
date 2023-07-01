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
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::Menu)))
            // OnUpdate Systems
            .add_system(interact_with_button.in_set(OnUpdate(AppState::Menu)))
            // OnExit Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::Menu)));
    }
}




