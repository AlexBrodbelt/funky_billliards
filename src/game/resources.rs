use bevy::{
    prelude::*,
};

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct CursorPosition{
    pub x: f32,
    pub y: f32
}

