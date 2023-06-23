use bevy::{
    prelude::*,
};

use crate::game::{ball::components::Ball, resources::{Player}};

// This resource tracks the game's score
#[derive(Resource)]
pub struct Scoreboard {
    pub player_1_score: usize,
    pub player_2_score: usize,
}


impl Scoreboard {
    pub fn pocket(&mut self, active_player: &Player, ball: &Ball) {
        match active_player {
            Player::One => {
                self.player_1_score += ball.score();
            },
            Player::Two => {
                self.player_2_score += ball.score();
            },
        }
    }
}
