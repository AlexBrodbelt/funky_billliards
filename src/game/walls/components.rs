use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

use crate::config::*;



#[derive(Component)]
pub enum Wall {
    Left,
    Right,
    Bottom,
    Top,
}

/// Which side of the arena is this wall located on?
impl Wall {
    pub fn position(&self) -> Vec2 {
        match self {
            Wall::Left => Vec2::new(LEFT_WALL, 0.),
            Wall::Right => Vec2::new(RIGHT_WALL, 0.),
            Wall::Bottom => Vec2::new(0., BOTTOM_WALL),
            Wall::Top => Vec2::new(0., TOP_WALL),
        }
    }
    
    // pub fn size(&self) -> Vec2 {
    //     let arena_height = TOP_WALL - BOTTOM_WALL;
    //     let arena_width = RIGHT_WALL - LEFT_WALL;
    //     // Make sure we haven't messed up our constants
    //     assert!(arena_height > 0.0);
    //     assert!(arena_width > 0.0);
        
    //     match self {
    //         Wall::Left | Wall::Right => {
    //             Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
    //         }
    //         Wall::Bottom | Wall::Top => {
    //             Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
    //         }
    //     }
    // }
    
    pub fn dimensions(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);
        
        match self {
            Wall::Left | Wall::Right => {
                Vec2::new(WALL_THICKNESS / 2.0, arena_height / 2.0 - GAP_BETWEEN_POCKET_AND_WALL)
            }
            Wall::Bottom | Wall::Top => {
                Vec2::new(arena_width / 2.0  - GAP_BETWEEN_POCKET_AND_WALL, WALL_THICKNESS / 2.0)
            }
        }
    }   
}


#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    // sprite_bundle: SpriteBundle,
    collider: Collider,
    rigid_body : RigidBody,
    restitution_coefficient: Restitution,
    wall: Wall,
    collision_group: CollisionGroups
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic

     // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    // translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
            //         scale: location.size().extend(1.0),
    pub fn new(wall: Wall,  meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> WallBundle {
        WallBundle {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(2.0 * (wall.dimensions())).into())
                    .into(),
                material: materials.add(ColorMaterial::from(WALL_COLOR)),
                transform: Transform::from_translation(wall.position().extend(1.0)),
                ..default()
            },
            collider: Collider::cuboid(wall.dimensions().x, wall.dimensions().y),
            collision_group: CollisionGroups::new( Group::GROUP_1, Group::GROUP_1),
            rigid_body: RigidBody::Fixed,
            restitution_coefficient:  Restitution::coefficient(0.95),
            wall
        }
    }
}