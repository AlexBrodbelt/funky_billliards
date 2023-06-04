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
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pocket: Pocket,
    collider: Collider,
    rigid_body : RigidBody,
    sensor: Sensor,
    active_events: ActiveEvents
}

impl PocketBundle {
    pub fn new(pocket: Pocket, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> PocketBundle {
        PocketBundle {
            material_mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(POCKET_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(POCKET_COLOR)),
                    transform: Transform::from_translation(pocket.position().extend(0.9)),
                    ..default()
            },
            pocket: pocket,
            // collider: ColliderBuilder {
            //     shape: SharedShape::ball(POCKET_RADIUS- BALL_RADIUS),
            //     mass_properties: ColliderMassProps::default(),
            //     friction: Self::default_friction(),
            //     restitution: 0.0,
            //     position: Isometry::identity(),
            //     is_sensor: true,
            //     user_data: 0,
            //     collision_groups: InteractionGroups::all(),
            //     solver_groups: InteractionGroups::all(),
            //     friction_combine_rule: CoefficientCombineRule::Average,
            //     restitution_combine_rule: CoefficientCombineRule::Average,
            //     active_collision_types: ActiveCollisionTypes::default(),
            //     active_hooks: ActiveHooks::empty(),
            //     active_events: ActiveEvents::empty(),
            //     enabled: true,
            //     contact_force_event_threshold: 0.0,
            // }.into(),
            collider: Collider::ball(POCKET_RADIUS - BALL_RADIUS),
            rigid_body: RigidBody::Fixed,
            sensor: Sensor,
            active_events: ActiveEvents::COLLISION_EVENTS,
        }
    }
}