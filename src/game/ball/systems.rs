use bevy::{
    prelude::*,
    input::{
        mouse::{
        MouseWheel,
        MouseButtonInput
        },
    },
};
use bevy_rapier2d::prelude::*;

use crate::{
    resources::CursorPosition,
    config::{LEFT_WALL, RIGHT_WALL, BOTTOM_WALL, TOP_WALL}, game::GameSetupState};

use crate::game::ball::{
    components::*,
    CueBallState,
};

// Balls
pub fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    
}

pub fn spawn_cueball(
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

// pub fn strike_cueball(
//     mut cueball_query: Query<(&Transform, &mut Velocity), With<CueBall>>,
//     mut mouse_wheel: EventReader<MouseWheel>,
//     cursor_position: Res<CursorPosition>,
// ) {
//     let Ok((transform, mut velocity)) = cueball_query.get_single_mut() else {
//         return;
//     };
    
//     let new_velocity = cursor_position.0 - transform.translation.truncate();

//     *velocity = Velocity::linear(new_velocity);

// }
/// click to set the cue ball initial position
pub fn set_cueball(
    mut commands: Commands,
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cueball_query: Query<&mut Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
) {
    if let Some(button_pressed) = mouse_button_input.iter().last() {
        info!("in set cueball system {:?}", &button_pressed);
        if let Ok(mut cueball_position) = cueball_query.get_single_mut() {
            let mut new_cueball_position = cursor_position.0;
            // making sure it doesn't cause the paddle to leave the arena
            new_cueball_position = new_cueball_position.clamp(Vec2::new(LEFT_WALL, BOTTOM_WALL), Vec2::new(RIGHT_WALL, TOP_WALL));
            cueball_position.translation = new_cueball_position.extend(1.0);

            // Change CueBallState to InPlay
            commands.insert_resource(NextState(Some(CueBallState::InPlay)));
        } else {
            println!("multiple ball entities have been spawned")
        }          
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