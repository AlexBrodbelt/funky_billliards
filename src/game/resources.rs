use std::time::Duration;

use bevy::prelude::*;

use crate::config::{WALL_VERTEX_BUFFER, WALL_INDEX_BUFFER};

#[derive(Resource)]
pub struct CollisionSound(pub Handle<AudioSource>);

#[derive(Resource, Debug)]
pub struct TableStatus {
    pub cue_stick_status: CueStickStatus,
    pub cue_ball_status: CueBallStatus,
    pub wall_status: WallStatus,
    pub pocket_status: PocketStatus,
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

/// struct containing relevant information of the wall vertices and which vertices are connected
#[derive(Debug)]
pub struct WallStatus {
    pub vertex_buffer: Vec<Vec2>,
    pub index_buffer: Vec<[u32; 2]>,
}

impl Default for WallStatus {
    fn default() -> Self {
        Self { 
            vertex_buffer: WALL_VERTEX_BUFFER.to_vec(),
            index_buffer: WALL_INDEX_BUFFER.to_vec()
        }
    }
}

impl WallStatus {
    pub fn clear_buffers(&mut self) {
        self.index_buffer.clear();
        self.vertex_buffer.clear();
    }
}

#[derive(Debug)]
pub struct PocketStatus {
    vertex_buffer: Vec<Vec2>,
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
            wall_status: WallStatus {
                vertex_buffer: Vec::new(),
                index_buffer: Vec::new(),
            },
            pocket_status: PocketStatus { 
                vertex_buffer: Vec::new(), 
            }
        }
    }
}

impl TableStatus {
    pub fn clear_wall_buffers(&mut self) {
        self.wall_status.clear_buffers()
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

#[derive(Component, Default, Debug)]
pub enum Player {
    #[default]
    One,
    Two,
}



