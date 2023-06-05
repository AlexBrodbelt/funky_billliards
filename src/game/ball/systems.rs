use bevy::{
    prelude::*,
    window::PrimaryWindow,
    input::mouse::MouseButton::*,
};
use bevy_rapier2d::prelude::*;

use crate::game::systems::get_cursor_position;

use super::components::*;

// Balls
pub fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>, 
) {
    commands.spawn(
        (
            BallBundle::new(Ball::White,  &mut meshes, &mut materials),
        CueBall
        )
    );
    // commands.spawn(BallBundle::new(Ball::Yellow, &mut meshes, &mut materials));
    // commands.spawn(BallBundle::new(Ball::Green,  &mut meshes, &mut materials));
    // commands.spawn(BallBundle::new(Ball::Blue,   &mut meshes, &mut materials));
    // commands.spawn(BallBundle::new(Ball::Pink,   &mut meshes, &mut materials));
    // commands.spawn(BallBundle::new(Ball::Brown,  &mut meshes, &mut materials));
    // commands.spawn(BallBundle::new(Ball::Black,  &mut meshes, &mut materials));
    for level in 0..5  {
        for index in 0..=level {
            commands.spawn(BallBundle::new(Ball::Red(RedBallIdentifier::new(level, index)), &mut meshes, &mut materials));
        }
    }
}


pub fn strike_cue_ball(
    mut cue_ball_query: Query<(&Transform, &mut Velocity), With<CueBall>>,
    mut cursor: EventReader<CursorMoved>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>
) {
    let Some(cursor_position) = get_cursor_position(cursor, primary_window_query) else {
        return;
    };

    let Ok((transform, mut velocity)) = cue_ball_query.get_single_mut() else {
        return;
    };
    
    let new_velocity = cursor_position - transform.translation.truncate();

    *velocity = Velocity::linear(new_velocity);
}

pub fn set_cue_ball(
    mut cue_ball_query: Query<&mut Transform, With<CueBall>>,
    mut cursor: EventReader<CursorMoved>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_input: Res<Input<MouseButton>>
) {
    let Some(cursor_position) = get_cursor_position(cursor, primary_window_query) else {
        return;
    };

    if mouse_input.pressed(Left) || mouse_input.pressed(Middle) || mouse_input.pressed(Right)  {
        let Ok(mut cue_ball_position) = cue_ball_query.get_single_mut() else {
            return;
        };
        cue_ball_position.translation = cursor_position.extend(1.0);
    }
}

pub fn despawn_balls(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
) {
    for ball in &ball_query {
        commands.entity(ball).despawn();
    }
}