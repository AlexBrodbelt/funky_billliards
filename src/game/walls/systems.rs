use bevy::prelude::*;

use super::components::*;

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>, 
) {
    // Walls
    commands.spawn(WallBundle::new(Wall::Left, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(Wall::Right, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(Wall::Bottom, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(Wall::Top, &mut meshes, &mut materials));
}

pub fn despawn_walls(
    mut commands: Commands,
    walls_query: Query<Entity, With<Wall>>,
) {
    for wall in &walls_query {
        commands.entity(wall).despawn();
    }
}