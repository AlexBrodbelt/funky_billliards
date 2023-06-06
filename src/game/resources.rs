use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct CursorPosition{
    pub x: f32,
    pub y: f32,
}

