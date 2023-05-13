use bevy::{
    prelude::*,
    utils::HashMap,
};

// #[derive(Resource)]
// pub struct SpriteDataList{
//     info : HashMap<Ball, (Position, Velocity)>,
// }

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub score: usize,
}

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

