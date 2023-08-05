use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::config::*;

use super::systems::{build_wall_shape_bundle, build_wall_collider};


#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    name: Name,
    shape_bundle: ShapeBundle,
    // sprite_bundle: SpriteBundle,
    collider: Collider,
    rigid_body : RigidBody,
    restitution_coefficient: Restitution,
    wall: Wall,
    collision_group: CollisionGroups,
    fill: Fill,
    stroke: Stroke
}

impl Default for WallBundle {
    fn default() -> Self {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(BOTTOM_LEFT_CORNER);
        path_builder.line_to(TOP_LEFT_CORNER);
        path_builder.line_to(TOP_RIGHT_CORNER);
        path_builder.line_to(BOTTOM_RIGHT_CORNER);
        path_builder.line_to(BOTTOM_LEFT_CORNER);
        path_builder.close();
        let path = path_builder.build();
        WallBundle {
            name: Name::new("Wall"),
            shape_bundle: ShapeBundle {
                path,
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            stroke: Stroke::new(WALL_COLOR, WALL_THICKNESS),
            fill: Fill::color(PLAY_FIELD_COLOR),
            collider: Collider::polyline(
                WALL_VERTEX_BUFFER.to_vec()
                                            .iter()
                                            .map(|&v| {
                                                v * 0.95
                                            })
                                            .collect::<Vec<Vec2>>(),
                Some(WALL_INDEX_BUFFER.to_vec()),
            ),
            collision_group: CollisionGroups::new( Group::GROUP_1, Group::GROUP_1),
            rigid_body: RigidBody::Fixed,
            restitution_coefficient:  Restitution::coefficient(0.95),
            wall: Wall,
        }
    }
}

impl WallBundle {
    pub fn new(vertex_buffer: Vec<Vec2>,  maybe_index_buffer: Option<Vec<[u32; 2]>>) -> Self {
        Self {
        shape_bundle:  build_wall_shape_bundle(&vertex_buffer), //ShapeBundle::default(),
        collider: build_wall_collider(vertex_buffer, maybe_index_buffer), //Collider::polyline(Vec::new(), None),
        stroke: Stroke::new(WALL_COLOR, WALL_THICKNESS),
        fill: Fill::color(PLAY_FIELD_COLOR),
        ..default() 
        }   
    }
}