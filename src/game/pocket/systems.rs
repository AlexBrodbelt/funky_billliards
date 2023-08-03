use bevy::{prelude::*, input::mouse::{MouseButtonInput}};
use bevy_rapier2d::prelude::*;

use super::components::*;

use crate::{game::{
    scoreboard::resources::*,
    resources::ActivePlayer,
    ball::components::Ball
    },
    resources::CursorPosition
};


pub fn set_pockets(
    mut commands: Commands,
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cursor_position: Res<CursorPosition>,
    // mut cue_ball_query: Query<&mut Transform, With<CueBall>>,
    // mut pocket_status: ResMut<PocketStatus>,
    // mut next_cue_ball_state: ResMut<NextState<CueBallState>>,
) {
    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        commands.spawn(PocketBundle::from_cursor_position(cursor_position.0, &mut meshes, &mut materials));
    }    
}

pub fn spawn_pockets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(PocketBundle::new(Pocket::BottomLeft, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::BotttomCenter, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::BottomRight, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopLeft, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopCenter, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopRight, &mut meshes, &mut materials ));
}

pub fn pocket_condition(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    rapier_context: Res<RapierContext>,
    active_player: Res<ActivePlayer>,
    pocket_query: Query<Entity, With<Pocket>>,
    ball_query: Query<&Ball, With<Ball>>,
) {
    for pocket in pocket_query.iter() {
        for (entity1, entity2, intersecting) in rapier_context.intersections_with(pocket){
            if intersecting {

                // if Ok then entity1 is the ball entity otherwise entity2 is the ball entity, the pocket entity is the alternative entity option
                let (entity, ball_type) = match ball_query.get(entity1) {
                    Ok(ball_type) => (entity1, ball_type),
                    Err(_) => (entity2, ball_query.get(entity2).unwrap()),
                };

                scoreboard.pocket(&active_player.0, ball_type);
                commands.entity(entity).despawn(); 
                             
            }
        }
    }
}

// pub fn despawn_pockets( // idea make despawning generic function that takes in the component to despawn
//     mut commands: Commands,
//     pocket_query: Query<Entity, With<Pocket>>,
// ) {
//     for pocket in &pocket_query {
//         commands.entity(pocket).despawn();
//     }
// }

