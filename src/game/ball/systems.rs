use bevy::{
    prelude::*,
    input::mouse::MouseButtonInput,
};

use crate::{
    resources::CursorPosition,
    config::{LEFT_WALL, RIGHT_WALL, BOTTOM_WALL, TOP_WALL}, game::GameSetUpState,
};

use crate::game::resources::TableStatus;

use crate::game::ball::{
    components::*,
    CueBallState,
};

// Balls
pub fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
    // asset_server: Res<AssetServer>, 
) {
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
    next_game_setup_state.set(GameSetUpState::CueBallSetup);    
}

pub fn spawn_cue_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(
        (
            BallBundle::new(Ball::White,  &mut meshes, &mut materials),
            CueBall
        )
    ); 
}

/// click to set the cue ball initial position
pub fn set_cue_ball(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cue_ball_query: Query<&mut Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
    mut table_status: ResMut<TableStatus>,
    mut next_cue_ball_state: ResMut<NextState<CueBallState>>,
) {
    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        if let Ok(mut cue_ball_position) = cue_ball_query.get_single_mut() {
            let mut new_cue_ball_position = cursor_position.0;
            // Making sure the ball does not leave the arena
            new_cue_ball_position = new_cue_ball_position.clamp(Vec2::new(LEFT_WALL, BOTTOM_WALL), Vec2::new(RIGHT_WALL, TOP_WALL));
            // Set cue ball initial position resource
            table_status.cue_ball_status.initial_position = Some(new_cue_ball_position);
            // Set the position of the cue ball
            cue_ball_position.translation = new_cue_ball_position.extend(1.0);
            // Change CueBallState to InPlay
            next_cue_ball_state.set(CueBallState::InPlay);
        } else {
            println!("either none or multiple ball entities have been spawned");
        }          
    }
}

pub fn despawn_balls(
    mut commands: Commands,
    ball_query: Query<Entity, (With<Ball>,)>,
) {
    for ball in &ball_query {
        commands.entity(ball).despawn();
    }
}