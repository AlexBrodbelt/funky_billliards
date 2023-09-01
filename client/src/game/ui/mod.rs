use bevy::prelude::*;

use self::{
    wall_set_up_menu::WallSetUpMenuPlugin,
    pocket_set_up_menu::PocketSetUpMenuPlugin
};

pub mod pocket_set_up_menu;
pub mod wall_set_up_menu;
pub mod styles;
pub mod components;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                ( 
                    PocketSetUpMenuPlugin,
                    WallSetUpMenuPlugin
                )
            );
    }
}
