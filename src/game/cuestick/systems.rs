use bevy::{
    prelude::*, input::mouse::{MouseButtonInput, MouseWheel, self},

};
use bevy_rapier2d::prelude::ExternalForce;

use crate::{config::*, resources::CursorPosition, game::{ball::components::CueBall, GameSetupState}, AppState};

use super::{
    components::*,
    resources::StrikeForce
};

pub fn spawn_cuestick(
    mut commands: Commands,
    cueball_query: Query<&Transform, With<CueBall>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    let cueball_transform = cueball_query.single();
    commands.spawn(CueStickBundle::new(cueball_transform ,meshes, materials));
    commands.insert_resource(StrikeForce::default());
}

pub fn despawn_cuestick(
    mut commands: Commands,
    cuestick_query: Query<Entity, With<CueStick>>,
) {   
    let cue_stick_entity = cuestick_query.single();
    commands.entity(cue_stick_entity).despawn();
    commands.remove_resource::<StrikeForce>();
}

pub fn set_cuestick(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cuestick_query: Query<&mut Transform, (With<CueStick>, Without<CueBall>)>, // without filter is necessary as there could be an entity with CueBall and CueStick component
    cueball_query: Query<&Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
) {
    let mut cuestick_transform = cuestick_query.single_mut();
    let cueball_transform = cueball_query.single();
    let diff = cursor_position.0 - cueball_transform.translation.truncate(); 

    // Calculate the new position for the cuestick adjacent to the cueball
    let diff_normalized = match diff.try_normalize() {
        Some(diff_normalized) => diff_normalized,
        None => Vec2::X,
    };
    let new_cuestick_translation = (BALL_RADIUS + GAP_BETWEEN_CUESTICK_AND_CUEBALL + CUESTICK_SIZE.x) * diff_normalized + cueball_transform.translation.truncate();

    // Calculate the angle of the cuestick      
    let new_cuestick_angle = Vec2::X.angle_between(diff_normalized);

    // Update the cuestick transform when mouse is clicked

    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        cuestick_transform.translation = new_cuestick_translation.extend(1.0);
        cuestick_transform.rotation = Quat::from_rotation_z(new_cuestick_angle);
    }
}

/// Drag back with two fingers to set the force applied to the cuestick.
pub fn strike_cueball(
    mut commands: Commands,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cuestick_query: Query<(&Transform, &mut ExternalForce), With<CueStick>>,
    mut strike_impulse: ResMut<StrikeForce>,
) { 
    for mouse_wheel_event in mouse_wheel_events.iter() {
        strike_impulse.0 += mouse_wheel_event.y.abs();
    }
    // If the mouse is not scrolled any further in the next frame then strike the cue ball and change to the next state
    if !strike_impulse.is_changed() && strike_impulse.0 != 0.0  {
        let (cuestick_transform, mut cuestick_external_force) = cuestick_query.single_mut();
        let impulse = strike_impulse.0;
        let (axis, angle) = cuestick_transform.rotation.to_axis_angle(); // how to convert from quaternion to angle?
        assert_eq!(axis, Vec3::Z);
        cuestick_external_force.force = impulse * Vec2::from_angle(angle);
        commands.insert_resource(NextState(Some(AppState::Game)));
    }
}


// pub fn set_cue_ball(
//     mut commands: Commands,
//     mut mouse_button_input: EventReader<MouseButtonInput>,
//     mut cue_ball_query: Query<&mut Transform, With<CueBall>>,
//     primary_window_query: Query<&Window, With<PrimaryWindow>>,
//     cursor_position: Res<CursorPosition>,
// ) {
//     if let Some(button_pressed) = mouse_button_input.iter().last() {
//         info!("{:?}", &button_pressed);
//         if let Ok(mut cue_ball_position) = cue_ball_query.get_single_mut() {
//             cue_ball_position.translation = cursor_position.0.extend(1.0);
//             commands.insert_resource(NextState(Some(CueBallState::InPlay)));
//         }           
//     }
// }