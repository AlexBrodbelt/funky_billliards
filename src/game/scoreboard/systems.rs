use bevy::prelude::*;

use crate::game::resources::Player;

use super::{resources::*, components::ScoreboardBundle};

pub fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.insert_resource( Scoreboard {
        player_1_score: 0,
        player_2_score: 0 
    });
    commands.spawn( ScoreboardBundle::new(&asset_server, Player::One));
    commands.spawn( ScoreboardBundle::new(&asset_server, Player::Two));
}

pub fn update_scoreboard(scoreboard_resource: Res<Scoreboard>, mut scoreboard_query: Query<(&Player, &mut Text)>) {
    for (player, mut text) in &mut scoreboard_query {
        match *player {
            Player::One => {
                text.sections[1].value = scoreboard_resource.player_1_score.to_string();
            },
            Player::Two => {
                text.sections[1].value = scoreboard_resource.player_2_score.to_string();
            },
        }
    }    
}

pub fn despawn_scoreboard(
    mut commands: Commands,
    scoreboard_text_query: Query<Entity, With<Text>>,
) {
    commands.remove_resource::<Scoreboard>();
    
    for  scoreboard_text_entity in  &scoreboard_text_query {
        commands.entity(scoreboard_text_entity).despawn();
    }

}

