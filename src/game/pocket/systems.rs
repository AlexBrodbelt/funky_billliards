use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

use crate::game::{scoreboard::resources::*, ball::components::Ball};

pub fn spawn_pockets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>, 
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
    pocket_query: Query<Entity, With<Pocket>>
) {
    for pocket in pocket_query.iter() {
        for (entity1, entity2, intersecting) in rapier_context.intersections_with(pocket){
            if intersecting {
                if entity1 == pocket {
                    commands.entity(entity2).despawn();
                } else {
                    commands.entity(entity1).despawn();
                }
                scoreboard.score += 1;
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