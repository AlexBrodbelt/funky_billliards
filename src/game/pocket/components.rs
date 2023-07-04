use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::{config::*, resources::CursorPosition};

#[derive(Component)]
pub enum Pocket {
    TopRight,
    TopCenter,
    TopLeft,
    BottomRight,
    BotttomCenter,
    BottomLeft,
    HandPlaced(PocketIdentifier),
}

pub struct PocketIdentifier {
    pub id: usize,
    pub position: Vec2,
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
            Pocket::HandPlaced(PocketIdentifier{ id, position}) => position,    
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Pocket::TopRight => "Top Right Pocket".to_string(),
            Pocket::TopCenter => "Top Center Pocket".to_string(),
            Pocket::TopLeft => "Top Left Pocket".to_string(),
            Pocket::BottomRight => "Bottom Right Pocket".to_string(),
            Pocket::BotttomCenter => "Botttom Center Pocket".to_string(),
            Pocket::BottomLeft => "Bottom Left Pocket".to_string(),
            Pocket::HandPlaced(PocketIdentifier{ id, position}) => format!("pocket {} at position {}", id, position),
        }
    }
}

impl From<&Pocket> for Name {
    fn from(pocket: &Pocket) -> Name {
        Name::new(pocket.name())        
    }
}

#[derive(Bundle)]
pub struct PocketBundle {
    active_events: ActiveEvents,
    collider: Collider,
    collision_group: CollisionGroups,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    name: Name,
    pocket: Pocket,
    rigid_body : RigidBody,
    sensor: Sensor,
}

impl PocketBundle {

    /// Returns a [`PocketBundle`] given a [`Pocket`] variant
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
            name: Name::from(&pocket),
            pocket,
            rigid_body: RigidBody::Fixed,
            sensor: Sensor,
        }
    }

    /// Returns a [`PocketBundle`] given the [`CursorPosition`] and sets it to said position 
    pub fn from_cursor_position(cursor_position: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {

        let pocket = Pocket::HandPlaced(PocketIdentifier { id: 0, position: cursor_position });

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
            name: Name::from(&pocket),
            pocket,
            rigid_body: RigidBody::Fixed,
            sensor: Sensor, 
        }  
    }
}

