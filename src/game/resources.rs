use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource, Debug)]
pub struct TableStatus {
    pub cue_stick_status: CueStickStatus,
    pub cue_ball_status: CueBallStatus,
}

#[derive(Debug)]
pub struct CueStickStatus {
    pub lifetime_timer: Timer,
    pub initial_position: Option<Vec2>, // position after wind up 
}

#[derive(Debug)]
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

/// Which player is currently active
#[derive(Resource)]
pub struct ActivePlayer(pub Player);
    

impl ActivePlayer {
    pub fn switch_player(&mut self) {
        self.0 = match self.0 {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }                
    }
}

impl Default for ActivePlayer {
    fn default() -> Self {
        ActivePlayer(Player::default())
    }
}

#[derive(Default, Debug)]
pub enum Player {
    #[default]
    One,
    Two,
}



