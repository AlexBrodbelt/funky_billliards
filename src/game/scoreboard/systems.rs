use bevy::prelude::*;

use crate::config::*;

use super::resources::*;

pub fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
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
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
    commands.insert_resource(Scoreboard { score: 0 })
}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

pub fn despawn_scoreboard(
    mut commands: Commands,
    scoreboard_text_query: Query<Entity, With<Text>>,
) {
    commands.remove_resource::<Scoreboard>();
    
    let scoreboard_text_entity = scoreboard_text_query.single();

    commands.entity(scoreboard_text_entity).despawn();

}

