use std::time::Duration;

use bevy::prelude::*;

use crate::config::{WALL_VERTEX_BUFFER, WALL_INDEX_BUFFER};




#[derive(Debug, Resource)]
pub struct CueStickStatus {
    pub lifetime_timer: Timer,
    pub initial_position: Option<Vec2>, // position after wind up 
}

impl Default for CueStickStatus {
    fn default() -> Self {
        CueStickStatus {
            lifetime_timer: Timer::new( Duration::from_secs(2), TimerMode::Repeating),
            initial_position: None
        }
    }
}


#[derive(Debug, Resource)]
pub struct CueBallStatus {
    pub initial_position: Option<Vec2>,
}

impl Default for CueBallStatus {
    fn default() -> Self {
        CueBallStatus {
            initial_position: None 
        }       
    }
}


/// struct containing relevant information of the wall vertices and which vertices are connected
#[derive(Debug, Resource)]
pub struct WallStatus {
    pub vertex_buffer: Vec<Vec2>,
    pub maybe_index_buffer: Option<Vec<[u32; 2]>>,
}

/// creates the default wall for the game
impl WallStatus {
    pub fn set_to_default(&mut self) {
        self.vertex_buffer =  WALL_VERTEX_BUFFER.to_vec();
        self.maybe_index_buffer = Some(WALL_INDEX_BUFFER.to_vec());   
    }
}

impl Default for WallStatus {
    fn default() -> Self {
        WallStatus {
            vertex_buffer: Vec::new(),
            maybe_index_buffer: Some(Vec::new()),
        }
    }
}

impl WallStatus {

    /// clears the wall buffers
    pub fn clear_buffers(&mut self) {
        if let Some(index_buffer) = &mut self.maybe_index_buffer {
            index_buffer.clear();
        } else {
            self.maybe_index_buffer = Some(Vec::new());
        }
        self.vertex_buffer.clear();
    }    
}


#[derive(Debug, Resource)]
pub struct PocketStatus {
    vertex_buffer: Vec<Vec2>,
}

impl Default for PocketStatus {
    fn default() -> Self {
        PocketStatus { 
            vertex_buffer: Vec::new(), 
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

#[derive(Component, Default, Debug)]
pub enum Player {
    #[default]
    One,
    Two,
}



