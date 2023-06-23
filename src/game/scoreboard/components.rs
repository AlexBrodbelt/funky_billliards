use bevy::prelude::*;

use crate::{config::*, game::resources::Player};

use super::styles::*;

#[derive(Bundle)]
pub struct ScoreboardBundle {
    text_bundle: TextBundle,
    player: Player,
}

impl ScoreboardBundle {
    pub fn new(asset_server: &Res<AssetServer>, player: Player) -> ScoreboardBundle {
        ScoreboardBundle {
            text_bundle: TextBundle::from_sections([
                TextSection::new(
                    format!("Player {:?} score: ", &player),
                    // "Player 2 score: ",
                    TextStyle {
                        font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts\\FiraMono-Medium.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCORE_COLOR,
                }),
                ])
                .with_style( match &player {
                    Player::One => SCORE_POSITION_1_STYLE, 
                    Player::Two => SCORE_POSITION_2_STYLE,
                }),
            player 
        }
    }
}