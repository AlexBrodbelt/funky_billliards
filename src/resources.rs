use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition(Vec2::default())
    }
}

// DEBUGGING PURPSOSES
#[derive(Resource)]
pub struct PeriodicTimer(pub Timer);
    