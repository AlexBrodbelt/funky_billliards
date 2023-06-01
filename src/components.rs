use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::{prelude::*};

// use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

use crate::config::*;

#[derive(Eq, PartialEq, Hash)]
pub struct RedBallIdentifier {
    level: usize,
    index: usize,
}

impl RedBallIdentifier {
    pub fn new(level: usize, index: usize) -> RedBallIdentifier {
        RedBallIdentifier { level, index }
    }
}

#[derive(Component, Eq, PartialEq, Hash)]
pub enum Ball {
    Black,
    White,
    Yellow,
    Red(RedBallIdentifier),
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

    pub fn position(&self) -> Vec2 {
        match *self {
            Ball::Black          => Vec2::new( (9.0 / 11.0) * RIGHT_WALL, 0.0),
            Ball::White          => Vec2::new(X_BAULK_D, 0.0),
            Ball::Yellow         => Vec2::new(X_BAULK_LINE, -R_BAULK_D),
            Ball::Red(RedBallIdentifier { level , index }) => {
                let x_offset = (1.0 / 2.0) * RIGHT_WALL ; 
                let y_offset = 0.0;
                Vec2::new(
                    x_offset + f32::sqrt( 3.0 )*(BALL_RADIUS + GAP_BETWEEN_BALLS / 2.0)*((level as f32) + 1.0),
                    y_offset + (2.0 * (index as f32) - (level as f32) ) * (0.5 + GAP_BETWEEN_BALLS)
                )
            },
            Ball::Blue           => Vec2::new(0.0, 0.0),
            Ball::Green          => Vec2::new(X_BAULK_LINE, R_BAULK_D),
            Ball::Pink           => Vec2::new(0.75 * TABLE_WIDTH, 0.0),
            Ball::Brown          => Vec2::new(X_BAULK_LINE, 0.0),
        }
    }

    pub fn velocity(&self) -> Vec2 {
        match *self {
            Ball::White => Vec2::new(600.0, 10.0),
            _ => Vec2::ZERO,
        }
    }
}

impl From<&Ball> for ColorMaterial {
    fn from(ball : &Ball) -> ColorMaterial {
        ColorMaterial::from(ball.color())       
    }
}

impl From<&Ball> for Transform {
    fn from(ball: &Ball) -> Transform {
        Transform::from_translation(ball.position()
                                        .extend(1.0))
    }
}

impl From<&Ball> for Velocity {
    fn from(ball: &Ball) -> Velocity {
        Velocity::linear(ball.velocity())
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    velocity: Velocity,
    collider: Collider,
    rigid_body: RigidBody,
    external_force: ExternalForce,
    damping: Damping,
    restitution_coefficient: Restitution,
}

impl BallBundle {
    pub fn new(ball: Ball, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> BallBundle {
        BallBundle {
        material_mesh_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(&ball)),
            transform: Transform::from(&ball),
            ..default()
        },
        velocity: Velocity::from(&ball),
        ball: ball,
        collider: Collider::ball(BALL_RADIUS),
        rigid_body: RigidBody::Dynamic,
        external_force: ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
            },
        damping: Damping {
            linear_damping: FRICTION_COEFFICIENT,
            angular_damping: FRICTION_COEFFICIENT,
            },
        restitution_coefficient: Restitution::coefficient(0.90), 
        }
    }
}

#[derive(Component)]
pub struct CueBall;



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


// #[derive(Default)]
// pub struct CollisionEvent;

// This bundle is a collection of the components that define a "wall" in our game

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
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
    pub fn dimensions(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);
        
        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS / 2.0, arena_height / 2.0 - GAP_BETWEEN_POCKET_AND_WALL)
            }
            WallLocation::Bottom | WallLocation::Top => {
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
    pub fn new(location: WallLocation,  meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> WallBundle {
        WallBundle {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(2.0 * (location.dimensions())).into())
                    .into(),
                material: materials.add(ColorMaterial::from(WALL_COLOR)),
                transform: Transform::from_translation(location.position().extend(1.0)),
                ..default()
            },
            collider: Collider::cuboid(location.dimensions()[0], location.dimensions()[1]),
            rigid_body: RigidBody::Fixed,
            restitution_coefficient:  Restitution::coefficient(0.95),
        }
    }
}

