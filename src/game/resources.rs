use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct TableStatus {
    pub cue_stick_status: CueStickStatus,
    pub cue_ball_status: CueBallStatus,
}

pub struct CueStickStatus {
    pub lifetime_timer: Timer,
    pub initial_position: Option<Vec2>, // position after wind up 
}

pub struct CueBallStatus {
    pub initial_position: Option<Vec2>,
}



impl Default for TableStatus {
    fn default() -> Self {
        TableStatus {
            cue_stick_status: CueStickStatus {
                lifetime_timer: Timer::new( Duration::from_secs(2), TimerMode::Repeating),
                initial_position: None
            },
            cue_ball_status: CueBallStatus {
                initial_position: None 
            },
        }
    }
}