use bevy::prelude::*;

#[derive(Event)]
pub enum WallSetUpEvent {
    SetWallVertex,
    ClearWall,
    Done,
    SpawnDefaultWall
}