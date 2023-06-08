use bevy::{
    prelude::*, input::mouse::MouseButtonInput,

};

use crate::{config::*, resources::CursorPosition, game::ball::components::CueBall};

use super::components::*;


pub fn spawn_cuestick(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    commands.spawn(CueStickBundle::new(meshes, materials));
}

pub fn despawn_cuestick(
    mut commands: Commands,
    cuestick_query: Query<Entity, With<CueStick>>,
) {   
    let cue_stick_entity = cuestick_query.single();
    commands.entity(cue_stick_entity).despawn();
}

pub fn set_cuestick(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut cuestick_query: Query<&mut Transform, With<CueStick>>,
    mut cueball_query: Query<&Transform, With<CueBall>>,
    cursor_position: Res<CursorPosition>,
) {
    let mut cuestick_transform = cuestick_query.single_mut();
    let cueball_transform = cueball_query.single_mut();
    // let cuestick_axis = cursor_position.0.extend(1.0) - cuestick_transform.translation;
    let diff = cueball_transform.translation - cursor_position.0.extend(1.0); 

    // // Calculate the new position for the cuestick adjacent to the cueball
    // let diff_normalized = match diff.try_normalize() {
    //     Some(diff_normalized) => diff_normalized,
    //     None => Vec3::X,
    // };
    // let new_cuestick_position = (BALL_RADIUS + GAP_BETWEEN_CUESTICK_AND_CUEBALL + (0.5 * CUESTICK_SIZE.x)) * diff_normalized;

    // // Calculate the angle of the cuestick      
    // let new_cuestick_angle = Quat::from_scaled_axis(cuestick_axis);

    // Update the cuestick transform when mouse is clicked

    if let Some(button_pressed) = mouse_button_input.iter().last() {
        cuestick_transform.look_to(diff, Vec3::Z);
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