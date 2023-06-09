use bevy::{
    prelude::*, input::mouse::{MouseButtonInput, MouseWheel, self},

};
use bevy_rapier2d::prelude::ExternalForce;

use crate::{config::*, resources::CursorPosition, game::{ball::components::CueBall, GameSetupState}, AppState};

use super::{
    components::*,
    resources::StrikeImpulse
};

pub fn spawn_cuestick(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    commands.spawn(CueStickBundle::new(meshes, materials));
    commands.insert_resource(StrikeImpulse::default());
}

pub fn despawn_cuestick(
    mut commands: Commands,
    cuestick_query: Query<Entity, With<CueStick>>,
) {   
    let cue_stick_entity = cuestick_query.single();
    commands.entity(cue_stick_entity).despawn();
    commands.remove_resource::<StrikeImpulse>();
}

pub fn set_cuestick(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cuestick_query: Query<&mut Transform, With<CueStick>>,
    mut cueball_query: Query<&Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
) {
    let mut cuestick_transform = cuestick_query.single_mut();
    let cueball_transform = cueball_query.single_mut();
    let cuestick_axis = cursor_position.0.extend(1.0) - cuestick_transform.translation;
    let diff = cueball_transform.translation - cursor_position.0.extend(1.0); 

    // Calculate the new position for the cuestick adjacent to the cueball
    let diff_normalized = match diff.try_normalize() {
        Some(diff_normalized) => diff_normalized,
        None => Vec3::X,
    };
    let new_cuestick_translation = (BALL_RADIUS + GAP_BETWEEN_CUESTICK_AND_CUEBALL + (0.5 * CUESTICK_SIZE.x)) * diff_normalized;

    // Calculate the angle of the cuestick      
    let new_cuestick_angle = Quat::from_scaled_axis(cuestick_axis);

    // Update the cuestick transform when mouse is clicked

    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        cuestick_transform.translation = new_cuestick_translation;
        cuestick_transform.rotation = new_cuestick_angle;
        // cuestick_transform.look_to(diff, Vec3::Z);
    }
}

/// Drag back with two fingers to set the force applied to the cuestick.
pub fn strike_cueball(
    mut commands: Commands,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cuestick_query: Query<(&Transform, &mut ExternalForce), With<CueStick>>,
    mut strike_impulse: ResMut<StrikeImpulse>,
) { 
    for mouse_wheel_event in mouse_wheel_events.iter() {
        strike_impulse.0 += mouse_wheel_event.y.abs();
    }
    // If the mouse is not scrolled any further in the next frame then strike the cue ball and change to the next state
    if !strike_impulse.is_changed() && strike_impulse.0 != 0.0  {
        let (cuestick_transform, mut cuestick_external_force) = cuestick_query.single_mut();
        // cuestick_external_force.force = cuestick_transform.rotation; // how to convert from quaternion to angle?
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