use bevy::prelude::*;

use super::components::*;

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>, 
) {
    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Right, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Bottom, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Top, &mut meshes, &mut materials));
}