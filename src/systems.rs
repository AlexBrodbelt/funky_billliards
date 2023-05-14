use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle, 
    utils::HashMap,
};

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

    let mut initial_data = HashMap::<Ball, (Position, Velocity)>::new();
    initial_data.insert(
        Ball::White,
        (
            Position::new(X_BAULK_D, 0.0),
            Velocity::new(400.0, 0.0))
    );
    // initial_data.insert(
    //     Ball::Green,
    //     (
    //         Position::new(X_BAULK_LINE, Y_BAULK_D),
    //         Velocity::new(0.0, 0.0)
    //     )
    // );
    // initial_data.insert(
    //     Ball::Brown,
    //     (
    //         Position::new(X_BAULK_LINE, 0.5 * TABLE_HEIGHT),
    //         Velocity::new(0.0, 0.0)
    //     )
    // );
    // initial_data.insert(
    //     Ball::Yellow,
    //     (
    //         Position::new(X_BAULK_LINE, 2.0 * Y_BAULK_D),
    //         Velocity::new(0.0, 0.0)
    //     )
    // );
    initial_data.insert(
        Ball::Blue,
        (
            Position::new( 0.0, 0.0),
            Velocity::new(0.0, 0.0)
        )
    );
    // initial_data.insert(
    //     Ball::Pink,
    //     (
    //         Position::new( 0.75 * TABLE_WIDTH, 0.5 * TABLE_HEIGHT),
    //         Velocity::new(0.0, 0.0)
    //     )
    // );
    // initial_data.insert(
    //     Ball::Black,
    //     (
    //         Position::new(0.875 * TABLE_WIDTH, 0.5 * TABLE_HEIGHT),
    //         Velocity::new(0.0, 0.0)
    //     )
    // );
    // // Red Balls
    // let x_offset = 0.75 * TABLE_WIDTH;
    // let y_offset = 0.5 * TABLE_HEIGHT;

    // for level in 0..5 {
    //     for index in 0..=level {
    //         initial_data.insert(
    //             Ball::Red(5 * level + index),
    //             (
    //                 Position::new(
    //                     x_offset + f32::sqrt(3.0)*(0.5 + GAP_BETWEEN_BALLS)*((level as f32) + 1.0),
    //                     y_offset + (2.0 *(index as f32) - (level as f32)) * (0.5 + GAP_BETWEEN_BALLS),
    //                 ),
    //                 Velocity::new(0.0, 0.0)
    //             )
    //         );

    //     }
    // }

    // initialise positions of the balls
    for (ball,  (Position(position), velocity)) in initial_data.into_iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(ball.color())),
                transform: Transform::from_translation(position).with_scale(BALL_SIZE),
                ..default()
            },
            ball,
            velocity,
        ));
    }

    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Circle::default().into()).into(),
    //         material: materials.add(ColorMaterial::from(Ball::White)),
    //         transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
    //         ..default()
    //     },
    //     Ball::White,
    //     Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    // ));
    



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

    // // Bricks
    // // Negative scales result in flipped sprites / meshes,
    // // which is definitely not what we want here
    // assert!(BRICK_SIZE.x > 0.0);
    // assert!(BRICK_SIZE.y > 0.0);

    // let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    // let bottom_edge_of_bricks = paddle_y + GAP_BETWEEN_PADDLE_AND_BRICKS;
    // let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    // assert!(total_width_of_bricks > 0.0);
    // assert!(total_height_of_bricks > 0.0);

    // // Given the space available, compute how many rows and columns of bricks we can fit
    // let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    // let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    // let n_vertical_gaps = n_columns - 1;

    // // Because we need to round the number of columns,
    // // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    // let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    // let left_edge_of_bricks = center_of_bricks
    //     // Space taken up by the bricks
    //     - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
    //     // Space taken up by the gaps
    //     - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // // In Bevy, the `translation` of an entity describes the center point,
    // // not its bottom-left corner
    // let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    // let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    // for row in 0..n_rows {
    //     for column in 0..n_columns {
    //         let brick_position = Vec2::new(
    //             offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
    //             offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
    //         );

    //         // brick
    //         commands.spawn((
    //             SpriteBundle {
    //                 sprite: Sprite {
    //                     color: BRICK_COLOR,
    //                     ..default()
    //                 },
    //                 transform: Transform {
    //                     translation: brick_position.extend(0.0),
    //                     scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             Brick,
    //             Collider,
    //         ));
    //     }
    // }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut ball_velocity, ball_transform) in &mut ball_query {
        
        // let (mut ball_velocity, ball_transform) = ball_query.single_mut();
        let ball_size = ball_transform.scale.truncate();

        // check collision with walls
        for (collider_entity, transform, maybe_brick) in &collider_query {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                // Bricks should be despawned and increment the scoreboard on collision
                if maybe_brick.is_some() {
                    scoreboard.score += 1;
                    commands.entity(collider_entity).despawn();
                }

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

pub fn velocity_system(mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query{
        velocity.x -= velocity.x * FRICTION_COEFFICIENT * TIME_STEP;
        velocity.y -= velocity.y * FRICTION_COEFFICIENT * TIME_STEP;

        if velocity.length_squared() < STOPPING_THRESHOLD {
            *velocity = Velocity(Vec2::ZERO);
        }
    } 
}

pub fn move_system(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

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