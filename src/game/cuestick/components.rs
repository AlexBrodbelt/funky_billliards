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
    collision_group: CollisionGroups,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    // velocity: Velocity,
    // rigid_body: RigidBody,
    // external_force: ExternalForce,
    // damping: Damping,
    // restitution_coefficient: Restitution,
}

impl CueStickBundle {
    pub fn new(mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> CueStickBundle {
        CueStickBundle { 
            cue_stick: CueStick,
            collider: Collider::cuboid(CUESTICK_SIZE.x, CUESTICK_SIZE.y),
            collision_group: CollisionGroups::new( Group::GROUP_2, Group::GROUP_2),
            material_mesh_bundle: MaterialMesh2dBundle { 
                mesh: meshes
                .add(shape::Quad::new(2.0 * CUESTICK_SIZE ).into())
                .into(),
                material: materials.add(ColorMaterial::from(CUESTICK_COLOR)), 
                transform: Transform { 
                    translation: CUESTICK_SIZE.extend(1.0),
                    rotation: Quat::IDENTITY,
                    ..Default::default()
                }, 
                 ..default()
            }  
        }
    }   
}

