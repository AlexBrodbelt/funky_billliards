use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::config::{WALL_COLOR, WALL_THICKNESS};

use super::components::*;

pub fn spawn_default_walls(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    // Walls
    // commands.spawn(DefaultWallBundle::new(Wall::Left, &mut meshes, &mut materials));
    // commands.spawn(DefaultWallBundle::new(Wall::Right, &mut meshes, &mut materials));
    // commands.spawn(DefaultWallBundle::new(Wall::Bottom, &mut meshes, &mut materials));
    // commands.spawn(DefaultWallBundle::new(Wall::Top, &mut meshes, &mut materials));
    commands.spawn(WallBundle::default());
}

pub fn build_wall(
    mut commands: Commands,
) {
    let mut path_builder = PathBuilder::new();
    let path = path_builder.build();

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 75., 0.),
            ..default()
        },
        Stroke::new(WALL_COLOR, WALL_THICKNESS),
        Fill::color(WALL_COLOR),
    ));
}

pub fn add_wall_segment(
    walls_query: Query<Entity, With<Wall>>,
) {

}

// pub fn spawn_walls(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
// commands.spawn(
//         ShapeBundle {
//             path,
//             transform: Transform::default(),
//             ..default()
//         }
//     );
// }

pub fn despawn_walls(
    mut commands: Commands,
    walls_query: Query<Entity, With<Wall>>,
) {
    for wall in &walls_query {
        commands.entity(wall).despawn();
    }
}



