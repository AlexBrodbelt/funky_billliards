use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

use crate::game::{
    scoreboard::resources::*,
    resources::ActivePlayer,
    ball::{components::Ball, self}};

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

pub fn despawn_pockets( // idea make despawning generic function that takes in the component to despawn
    mut commands: Commands,
    pocket_query: Query<Entity, With<Pocket>>,
) {
    for pocket in &pocket_query {
        commands.entity(pocket).despawn();
    }
}