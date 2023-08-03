use bevy::prelude::*;

#[derive(Component)]
pub struct WallSetUpMenu;

#[derive(Component)]
pub enum WallSetUpMenuButton {
    Clear,
    Done,
    Default,
}

#[derive(Component)]
pub struct CanvasButton;




