use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;

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