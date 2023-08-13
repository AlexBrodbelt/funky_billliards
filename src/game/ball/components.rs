use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_xpbd_2d::prelude::*;

use crate::{config::*, game::Layer};

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct RedBallIdentifier {
    level: usize,
    index: usize,
}

impl RedBallIdentifier {
    pub fn new(level: usize, index: usize) -> RedBallIdentifier {
        RedBallIdentifier { level, index }
    }
}

#[derive(Component, Eq, PartialEq, Hash, Debug)]
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
            // Ball::White => Vec2::new(600.0, 10.0),
            _ => Vec2::ZERO,
        }
    }

    pub fn score(&self) -> usize {
        match *self {
            Ball::Black => BLACK_SCORE,
            Ball::White => WHITE_SCORE,
            Ball::Yellow => YELLOW_SCORE,
            Ball::Red(_) => RED_SCORE,
            Ball::Blue => BLUE_SCORE,
            Ball::Green => GREEN_SCORE,
            Ball::Pink => PINK_SCORE,
            Ball::Brown => BROWN_SCORE,
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Ball::Black => "Black".to_string(),
            Ball::White => "White".to_string(),
            Ball::Yellow => "Yellow".to_string(),
            Ball::Red(RedBallIdentifier { level, index }) => format!("Red {} {}", level, index),
            Ball::Blue => "Blue".to_string(),
            Ball::Green => "Green".to_string(),
            Ball::Pink => "Pink".to_string(),
            Ball::Brown => "Brown".to_string(),
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



impl From<&Ball> for LinearVelocity {
    fn from(ball: &Ball) -> LinearVelocity {
        LinearVelocity(ball.velocity())
    }
}

impl From<&Ball> for Name {
    fn from(ball: &Ball) -> Name {
        Name::new(ball.name())
    }
}

/// Bundle of components for the Ball.
#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    collider: Collider,
    collision_group: CollisionLayers,
    linear_damping: LinearDamping,
    angular_damping: AngularDamping,
    // external_force: ExternalForce,
    name: Name,
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    restitution_coefficient: Restitution,
    rigid_body: RigidBody,
    velocity: LinearVelocity,
}


impl BallBundle {
    /// Given a Ball Type, the meshes and materials creates a new BallBundle
    pub fn new(ball: Ball, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> BallBundle {
        BallBundle {
            collider: Collider::ball(BALL_RADIUS),
            collision_group: if ball == Ball::White {
                    CollisionLayers::new([Layer::Wall] ,[Layer::CueStick, Layer::Wall] )
                } else {
                    CollisionLayers::new( [Layer::CueStick], [Layer::Wall])
                },
            linear_damping: LinearDamping(FRICTION_COEFFICIENT),
            angular_damping: AngularDamping(FRICTION_COEFFICIENT),
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(&ball)),
                transform: Transform::from(&ball),
                ..default()
            },
            name: Name::from(&ball),
            rigid_body: RigidBody::Dynamic,
            restitution_coefficient: Restitution::new(0.90), 
            velocity: LinearVelocity::from(&ball),
            ball,
        }
    }
}

#[derive(Component)]
pub struct CueBall;