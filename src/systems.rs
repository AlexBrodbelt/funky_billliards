use bevy::{
    prelude::*,
    window::PrimaryWindow,
    input::mouse::MouseButton::*,
};
use bevy_rapier2d::prelude::*;

use crate::config::*;
use crate::components::*; 
use crate::resources::*;


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

    // Balls
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

    // initialise positions of the balls
    // for ball in Ball::iter() {
    //     match ball {
    //         Ball::Red(_) => {
    //             for id in 0..15 {
    //             commands.spawn(BallBundle::new(Ball::Red(id), &mut meshes, &mut materials));
    //             }
    //         },
    //         _  => {
    //             commands.spawn(BallBundle::new(ball, &mut meshes, &mut materials));
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
    commands.spawn(WallBundle::new(WallLocation::Left, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Right, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Bottom, &mut meshes, &mut materials));
    commands.spawn(WallBundle::new(WallLocation::Top, &mut meshes, &mut materials));

    // Pockets
    commands.spawn(PocketBundle::new(Pocket::BottomLeft, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::BotttomCenter, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::BottomRight, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopLeft, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopCenter, &mut meshes, &mut materials ));
    commands.spawn(PocketBundle::new(Pocket::TopRight, &mut meshes, &mut materials ));
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
        // This prevents events staying active on the next frame.r
        collision_events.clear();
        audio.play(sound.0.clone());
    }
}

pub fn pocket_condition(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    rapier_context: Res<RapierContext>,
    pocket_query: Query<Entity, With<Pocket>>
) {
    for pocket in pocket_query.iter() {
        for (ball, _pocket, intersecting) in rapier_context.intersections_with(pocket){
            if intersecting {
                commands.entity(ball).despawn();
                scoreboard.score += 1;
            }
        }
    }
}

fn get_cursor_position(
    mut cursor: EventReader<CursorMoved>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>
) -> Option<Vec2> {
    let Ok(primary) = primary_window_query.get_single() else {
        return None;
    };
    let mut cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return None
    };
    cursor_position.x -= 0.5 * primary.width();
    cursor_position.y -= 0.5 * primary.height();
    Some(cursor_position)
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

    if (mouse_input.pressed(Left) || mouse_input.pressed(Middle) || mouse_input.pressed(Right))  {
        let Ok(mut cue_ball_position) = cue_ball_query.get_single_mut() else {
            return;
        };
    
        cue_ball_position.translation = cursor_position.extend(1.0);
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
