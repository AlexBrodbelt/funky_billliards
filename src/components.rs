use bevy::{
    prelude::*,
    // sprite::collide_aabb::{collide, Collision},
    // sprite::MaterialMesh2dBundle,
};

use crate::config::*;

#[derive(Component, Eq, Hash, PartialEq)]
pub enum Ball {
    Black,
    White,
    Yellow,
    Red(usize),
    Blue,
    Green,
    Pink,
    Brown,
}

impl Ball {
    pub fn color(&self) -> Color {
        match *self {
            Ball::Black => Color::rgb(0.0, 0.0, 0.0),
            Ball::White => Color::rgb(1.0, 1.0, 1.0),
            Ball::Yellow => Color::rgb(1.0, 0.68, 0.26),
            Ball::Red(_) => Color::rgb(1.0, 0.0, 0.0),
            Ball::Blue => Color::rgb(0.0, 0.0, 1.0),
            Ball::Green => Color::rgb(0.0, 1.0, 0.0),
            Ball::Pink => Color::rgb(1.0, 0.08, 0.58),
            Ball::Brown => Color::rgb(0.55, 0.27, 0.07),
        }
    }
}

impl From<Ball> for ColorMaterial {
    fn from(ball_color : Ball) -> ColorMaterial {
        ColorMaterial::from(ball_color.color())       
    }
}

#[derive(Component)]
pub struct Brick;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x : f32, y : f32) -> Velocity {
        Velocity(Vec2::new(x, y))
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Position(pub Vec3);

impl Position {
    pub fn new(x : f32, y : f32) -> Position {
        Position(Vec3::new(x, y, 1.0))
    }
}

#[derive(Component)]
pub struct Pocket;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}





