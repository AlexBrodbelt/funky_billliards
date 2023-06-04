use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::config::*;

#[derive(Component)]
pub struct CueStick;

#[derive(Bundle)]
pub struct CueStickBundle {
    cue_stick: CueStick,
    collider: Collider,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    // velocity: Velocity,
    // rigid_body: RigidBody,
    // external_force: ExternalForce,
    // damping: Damping,
    // restitution_coefficient: Restitution,
}

pub fn spawn_cue_stick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    todo!()
}