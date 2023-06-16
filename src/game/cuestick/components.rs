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
    // external_force: ExternalForce,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    rigid_body: RigidBody,
    velocity: Velocity,
    // damping: Damping,
    // restitution_coefficient: Restitution,
}

impl CueStickBundle {
    pub fn new(cueball_transform: &Transform, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> CueStickBundle {
        // position the cuestick to the right of the cueball
        let cuestick_position = cueball_transform.translation.truncate() - (BALL_RADIUS + CUESTICK_SIZE.x + GAP_BETWEEN_CUESTICK_AND_CUEBALL) * Vec2::X;
        CueStickBundle { 
            cue_stick: CueStick,
            collider: Collider::cuboid(CUESTICK_SIZE.x, CUESTICK_SIZE.y),
            collision_group: CollisionGroups::new( Group::GROUP_2, Group::GROUP_2),
            // external_force: ExternalForce { force: Vec2::ZERO, torque: 0.0 },
            material_mesh_bundle: MaterialMesh2dBundle { 
                mesh: meshes
                .add(shape::Quad::new(2.0 * CUESTICK_SIZE ).into())
                .into(),
                material: materials.add(ColorMaterial::from(CUESTICK_COLOR)), 
                transform: Transform { 
                    translation: cuestick_position.extend(1.0),
                    rotation: Quat::IDENTITY,
                    ..Default::default()
                }, 
                ..default()
            },
            rigid_body: RigidBody::KinematicVelocityBased,
            velocity: Velocity::zero(),
        }
    }   
}

