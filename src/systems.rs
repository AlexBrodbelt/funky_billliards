use bevy::{
    prelude::*,
};
use bevy_rapier2d::prelude::*;


use crate::config::*;
use crate::components::*; 
use crate::resources::*;

// use bevy::window::PrimaryWindow;



// Add the game's entities to our world
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds\\breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // // Paddle
    // let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, paddle_y, 0.0),
    //             scale: PADDLE_SIZE,
    //             ..default()
    //         },
    //         sprite: Sprite {
    //             color: PADDLE_COLOR,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Paddle,
    //     Collider,
    // ));

    
    commands.spawn(BallBundle::new(Ball::White, &mut meshes, &mut materials));
    commands.spawn(BallBundle::new(Ball::Green, &mut meshes, &mut materials));
    // initialise positions of the balls
    // for ball in Ball::iter() {
    //     match ball {
    //         Ball::Red(_) => {
    //             for id in 0..15 {
    //             commands.spawn(BallBundle::new(Ball::Red(id), meshes, materials));
    //             }
    //         },
    //         _  => {
    //             commands.spawn(BallBundle::new(ball, meshes, materials));
    //         }
    //     }
    // }

    



    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts\\FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    // Cuestick-Player
}




// pub fn move_system(mut query: Query<(&mut Transform, &Velocity)>) {
//     for (mut transform, velocity) in &mut query {
//         transform.translation.x += velocity.x * TIME_STEP;
//         transform.translation.y += velocity.y * TIME_STEP;
//     }
// }

pub fn play_collision_sound(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        // This prevents events staying active on the next frame.
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

// pub fn move_paddle(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut query: Query<&mut Transform, With<Paddle>>,
// ) {
//     let mut paddle_transform = query.single_mut();
//     let mut direction = 0.0;

//     if keyboard_input.pressed(KeyCode::Left) {
//         direction -= 1.0;
//     }

//     if keyboard_input.pressed(KeyCode::Right) {
//         direction += 1.0;
//     }

//     // Calculate the new horizontal paddle position based on player input
//     let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

//     // Update the paddle position,
//     // making sure it doesn't cause the paddle to leave the arena
//     let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
//     let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

//     paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
// }

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
