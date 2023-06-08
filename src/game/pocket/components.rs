use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::config::*;

#[derive(Component)]
pub enum Pocket {
    TopRight,
    TopCenter,
    TopLeft,
    BottomRight,
    BotttomCenter,
    BottomLeft,
}

impl Pocket {
    pub fn position(&self) -> Vec2 {
        match *self {
            Pocket::TopRight      => Vec2::new(RIGHT_WALL - GAP_BETWEEN_POCKET_AND_WALL, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::TopCenter     => Vec2::new(0.0, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::TopLeft       => Vec2::new(LEFT_WALL + GAP_BETWEEN_POCKET_AND_WALL, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::BottomRight   => Vec2::new(RIGHT_WALL - GAP_BETWEEN_POCKET_AND_WALL, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::BotttomCenter => Vec2::new(0.0, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::BottomLeft    => Vec2::new(LEFT_WALL + GAP_BETWEEN_POCKET_AND_WALL, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
        }
    }
}

#[derive(Bundle)]
pub struct PocketBundle {
    active_events: ActiveEvents,
    collider: Collider,
    collision_group: CollisionGroups,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pocket: Pocket,
    rigid_body : RigidBody,
    sensor: Sensor,
}

impl PocketBundle {
    pub fn new(pocket: Pocket, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> PocketBundle {
        PocketBundle {
            active_events: ActiveEvents::COLLISION_EVENTS,
            collider: Collider::ball(POCKET_RADIUS - BALL_RADIUS),
            collision_group: CollisionGroups::new( Group::GROUP_1, Group::GROUP_1),
            material_mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(POCKET_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(POCKET_COLOR)),
                    transform: Transform::from_translation(pocket.position().extend(0.9)),
                    ..default()
            },
            pocket,
            rigid_body: RigidBody::Fixed,
            sensor: Sensor,
        }
    }
}