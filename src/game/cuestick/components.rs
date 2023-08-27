use std::f32::consts::PI;

use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::*;

use crate::{config::*, game::Layer};

#[derive(Component)]
pub struct CueStick;

#[derive(Bundle)]
pub struct CueStickBundle {
    cue_stick: CueStick,
    collider: Collider,
    collision_group: CollisionLayers,
    // external_force: ExternalForce,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    name: Name,
    position: Position,
    rigid_body: RigidBody,
    rotation: Rotation,
    velocity: LinearVelocity,
    // damping: Damping,
    // restitution_coefficient: Restitution,
}

impl CueStickBundle {
    pub fn new(cueball_transform: &Transform, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> CueStickBundle {
        // position the cuestick to the right of the cueball
        let cuestick_position = cueball_transform.translation.truncate() - (BALL_RADIUS + CUESTICK_SIZE.x + GAP_BETWEEN_CUESTICK_AND_CUEBALL) * Vec2::X;
        CueStickBundle { 
            cue_stick: CueStick,
            collider: Collider::cuboid(2.0 * CUESTICK_SIZE.x, 2.0 * CUESTICK_SIZE.y),
            collision_group: CollisionLayers::new( [Layer::CueStick], [Layer::CueStick]),
            // external_force: ExternalForce { force: Vec2::ZERO, torque: 0.0 },
            material_mesh_bundle: MaterialMesh2dBundle { 
                mesh: meshes
                .add(shape::Quad::new(2.0 * CUESTICK_SIZE ).into())
                .into(),
                material: materials.add(ColorMaterial::from(CUESTICK_COLOR)), 
                transform: Transform { 
                    translation: cuestick_position.extend(1.1),
                    rotation: Quat::from_rotation_z(PI),
                    ..Default::default()
                }, 
                ..default()
            },
            name: Name::new("cue stick"),
            position: Position(cuestick_position),
            rigid_body: RigidBody::Kinematic,
            rotation: Rotation::ZERO,
            velocity: LinearVelocity::ZERO,
        }
    }   
}

