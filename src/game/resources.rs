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

