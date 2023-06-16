use std::time::Duration;

use bevy::{
    prelude::*, input::mouse::{MouseButtonInput, MouseWheel},

};
use bevy_rapier2d::prelude::Velocity;

use crate::{config::*, resources::CursorPosition, game::{ball::components::CueBall, GameState, resources::TableStatus}, AppState};

use super::{
    components::*,
    resources::WindUpDistance
};

pub fn spawn_cue_stick(
    mut commands: Commands,
    cue_ball_query: Query<&Transform, With<CueBall>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    let cue_ball_transform = cue_ball_query.single();
    commands.spawn(CueStickBundle::new(cue_ball_transform ,meshes, materials));
    commands.insert_resource(WindUpDistance::default());
}


/// manages the placement of the cue_stick given mouse position input.
pub fn set_cue_stick(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cue_stick_query: Query<&mut Transform, (With<CueStick>, Without<CueBall>)>, // without filter is necessary as there could be an entity with CueBall and CueStick component
    cue_ball_query: Query<&Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
) {
    let mut cue_stick_transform = cue_stick_query.single_mut();
    let cue_ball_transform = cue_ball_query.single();
    let diff = cursor_position.0 - cue_ball_transform.translation.truncate(); 

    // Calculate the new position for the cue_stick adjacent to the cue_ball
    let diff_normalized = match diff.try_normalize() {
        Some(diff_normalized) => diff_normalized,
        None => Vec2::X,
    };
    let new_cue_stick_translation = (BALL_RADIUS + GAP_BETWEEN_CUESTICK_AND_CUEBALL + CUESTICK_SIZE.x) * diff_normalized + cue_ball_transform.translation.truncate();

    // Calculate the angle of the cue_stick      
    let new_cue_stick_angle = Vec2::X.angle_between(diff_normalized);

    // Update the cue_stick transform when mouse is clicked

    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        cue_stick_transform.translation = new_cue_stick_translation.extend(1.0);
        cue_stick_transform.rotation = Quat::from_rotation_z(new_cue_stick_angle);
    }
}

/// When there wind up distance has been computed set the initial velocity of the cue ball.
/// Handle appropriate state transitions and resources.
pub fn strike_cue_ball(
    mut commands: Commands,
    mut cue_stick_query: Query<(&Transform, &mut Velocity), With<CueStick>>,
    wind_up_distance_resource: Res<WindUpDistance>,
    mut table_status: ResMut<TableStatus>,
) { 
    // If the mouse is not scrolled any further in the next frame then strike the cue ball and change to the next state
    if !wind_up_distance_resource.is_changed() && wind_up_distance_resource.0 != 0.0  {
        let (cue_stick_transform, mut cue_stick_velocity) = cue_stick_query.single_mut();
        let wind_up_distance = wind_up_distance_resource.0;
        let (axis, angle) = cue_stick_transform.rotation.to_axis_angle();
        // println!("axis {:?} - angle {:?} - force {:?}", axis, angle, wind_up_distance);
        // set the velocity of the cue stick
        cue_stick_velocity.linvel =  - (VELOCITY_SCALING * wind_up_distance).clamp(MIN_VELOCITY, MAX_VELOCITY) * Vec2::from_angle(axis.z * angle);
        // record initial position of the cue stick
        table_status.cue_stick_status.initial_position =  Some(cue_stick_transform.translation.truncate());
        println!("{:?}", cue_stick_velocity);
        commands.insert_resource(NextState(Some(AppState::Game)));
        commands.insert_resource(NextState(Some(GameState::ShotCooldown)))
    }
}

/// Drag back with two fingers to wind up the cue stick.
pub fn compute_wind_up_distance(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    cue_ball_query: Query<&Transform, (With<CueBall>, Without<CueStick>)>,
    mut cue_stick_query: Query<&mut Transform, With<CueStick>>,
    mut strike_force: ResMut<WindUpDistance>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        strike_force.0 += mouse_wheel_event.y.abs();
        if let (Ok(mut cue_stick_transform), Ok(cue_ball_transform)) = (cue_stick_query.get_single_mut(), cue_ball_query.get_single()) {
            let pull_back_displacement = (cue_stick_transform.translation - cue_ball_transform.translation).truncate().normalize() * mouse_wheel_event.y * PULL_BACK_DISPLACEMENT_CONVERSION_FACTOR;
            cue_stick_transform.translation += pull_back_displacement.extend(0.0);
        } else {
            println!("there is no cue sitick entity");
        }
    }
}

/// The force acting on the cue stick is proportional to the displacement with respect to the initial position of the cue ball.
/// The cue stick will despawn once it's position is past a threshold value relative to the cue ball's initial position. 
pub fn handle_cue_stick_motion(
    mut commands: Commands,
    mut cue_stick_query: Query<(Entity, &Transform, &mut Velocity), With<CueStick>>,
    mut table_status: ResMut<TableStatus>,
    time: Res<Time>,
) {
    if let Ok((cue_stick_entity, cue_stick_transform, mut cue_stick_velocity)) = cue_stick_query.get_single_mut() {
        if let Some(cue_ball_initial_position) = table_status.cue_ball_status.initial_position {
            // tick timer
            table_status.cue_stick_status.lifetime_timer.tick(time.delta());
            let cue_stick_distance_from_initial_cue_ball_position = (cue_stick_transform.translation.truncate() - cue_ball_initial_position).length();
            println!("cue stick initial distance from cue ball {:?}", cue_stick_distance_from_initial_cue_ball_position);

            // despawn cue stick condition
            if cue_stick_distance_from_initial_cue_ball_position < 0.125 * BALL_RADIUS || table_status.cue_stick_status.lifetime_timer.finished() {
                println!("timer ran out {:?}", table_status.cue_stick_status.lifetime_timer.finished());
                commands.entity(cue_stick_entity).despawn();
                // remove WindUpDistance resource
                commands.remove_resource::<WindUpDistance>();
    
                commands.insert_resource(NextState(Some(GameState::Playing)));
            }
        } else {
            println!("the cue ball has not been placed or the cue ball initial position resource is None");
        }        
    } else {
        println!("there is either no cue stick or there are too many cue sticks");
    } 
}