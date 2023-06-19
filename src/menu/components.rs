use bevy::prelude::*;

#[derive(Component)]
pub struct Menu;

#[derive(Component)]
pub enum MenuButton {
    Play,
    /// When pressed quits the game
    Quit,
    // /// When pressed the walls and pockets can be edited
    // Edit,
}



