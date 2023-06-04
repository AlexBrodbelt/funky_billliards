use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::components::*;

use crate::game::scoreboard::resources::*;

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
        for (ball, _pocket, intersecting) in rapier_context.intersections_with(pocket){
            if intersecting {
                commands.entity(ball).despawn();
                scoreboard.score += 1;
            }
        }
    }
}