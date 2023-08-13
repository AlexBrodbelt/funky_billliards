use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::*;

use crate::{config::*, game::Layer};

#[derive(Component)]
pub enum Pocket {
    Standard(StandardPocket),    
    HandPlaced(PocketIdentifier),
}

pub enum StandardPocket {
    TopRight,
    TopCenter,
    TopLeft,
    BottomRight,
    BotttomCenter,
    BottomLeft,
}

pub struct PocketIdentifier {
    pub id: usize,
    pub position: Vec2,
}

impl Pocket {
    pub fn position(&self) -> Vec2 {
        match *self {
            Pocket::Standard(StandardPocket::TopRight)      => Vec2::new(RIGHT_WALL - GAP_BETWEEN_POCKET_AND_WALL, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::Standard(StandardPocket::TopCenter)     => Vec2::new(0.0, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::Standard(StandardPocket::TopLeft)       => Vec2::new(LEFT_WALL + GAP_BETWEEN_POCKET_AND_WALL, TOP_WALL - GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::Standard(StandardPocket::BottomRight)   => Vec2::new(RIGHT_WALL - GAP_BETWEEN_POCKET_AND_WALL, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::Standard(StandardPocket::BotttomCenter) => Vec2::new(0.0, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::Standard(StandardPocket::BottomLeft)    => Vec2::new(LEFT_WALL + GAP_BETWEEN_POCKET_AND_WALL, BOTTOM_WALL + GAP_BETWEEN_POCKET_AND_WALL),
            Pocket::HandPlaced(PocketIdentifier{ id: _, position}) => position,    
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Pocket::Standard(StandardPocket::TopRight) => "Top Right Pocket".to_string(),
            Pocket::Standard(StandardPocket::TopCenter) => "Top Center Pocket".to_string(),
            Pocket::Standard(StandardPocket::TopLeft) => "Top Left Pocket".to_string(),
            Pocket::Standard(StandardPocket::BottomRight) => "Bottom Right Pocket".to_string(),
            Pocket::Standard(StandardPocket::BotttomCenter) => "Botttom Center Pocket".to_string(),
            Pocket::Standard(StandardPocket::BottomLeft) => "Bottom Left Pocket".to_string(),
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
    collision_group: CollisionLayers,
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
            collision_group: CollisionLayers::new( [Layer::CueStick, Layer::Wall], [Layer::Ball]),
            material_mesh_bundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(POCKET_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(POCKET_COLOR)),
                    transform: Transform::from_translation(pocket.position().extend(0.9)),
                    ..default()
            },
            name: Name::from(&pocket),
            pocket,
            rigid_body: RigidBody::Static,
            sensor: Sensor,
        }
    }

    /// Returns a [`PocketBundle`] given the [`CursorPosition`] and sets it to said position 
    pub fn from_vec(vector: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> Self {

        let pocket = Pocket::HandPlaced(PocketIdentifier { id: 0, position: vector });

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
            rigid_body: RigidBody::Static,
            sensor: Sensor, 
        }  
    }
}


