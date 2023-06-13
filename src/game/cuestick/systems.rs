use std::time::Duration;

use bevy::{
    prelude::*, input::mouse::{MouseButtonInput, MouseWheel},

};
use bevy_rapier2d::prelude::ExternalForce;

use crate::{config::*, resources::CursorPosition, game::{ball::components::CueBall, GameState}, AppState};

use super::{
    components::*,
    resources::{StrikeForce, CueStickLifetimeTimer}
};

pub fn insert_cue_stick_lifetime_timer(
    mut commands: Commands,
) {
    commands.insert_resource(
        CueStickLifetimeTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Once)
        }
    );
}

pub fn remove_cue_stick_lifetime_timer(
    mut commands: Commands
) {
    commands.remove_resource::<CueStickLifetimeTimer>();
}

pub fn spawn_cue_stick(
    mut commands: Commands,
    cue_ball_query: Query<&Transform, With<CueBall>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    let cue_ball_transform = cue_ball_query.single();
    commands.spawn(CueStickBundle::new(cue_ball_transform ,meshes, materials));
    commands.insert_resource(StrikeForce::default());
}

/// updates the timer which manages the cool down period for the cue_stick once it has been thrusted.
pub fn cue_stick_cooldown(
    time: Res<Time>,
    mut cue_stick_lifetime_timer: ResMut<CueStickLifetimeTimer>
) {
    cue_stick_lifetime_timer.timer.tick(time.delta());
}

/// despawns the cue_stick after the cooldown period has elapsed and modifies the state of the game:
/// AppState::GameSetup -> AppState::Game
pub fn despawn_cue_stick(
    mut commands: Commands,
    cue_stick_query: Query<Entity, With<CueStick>>,
    config: Res<CueStickLifetimeTimer>,
) { 
    if config.timer.finished() { 
        let cue_stick_entity = cue_stick_query.single();
        commands.entity(cue_stick_entity).despawn();
        commands.remove_resource::<StrikeForce>();

        commands.insert_resource(NextState(Some(GameState::Playing)));
    }  
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

/// Drag back with two fingers to set the force applied to the cue_stick.
pub fn strike_cue_ball(
    mut commands: Commands,
    mut cue_stick_query: Query<(&Transform, &mut ExternalForce), With<CueStick>>,
    strike_force: Res<StrikeForce>,
) { 
    // If the mouse is not scrolled any further in the next frame then strike the cue ball and change to the next state
    if !strike_force.is_changed() && strike_force.0 != 0.0  {
        let (cue_stick_transform, mut cue_stick_external_force) = cue_stick_query.single_mut();
        let force = strike_force.0;
        let (_axis, angle) = cue_stick_transform.rotation.to_axis_angle(); // how to convert from quaternion to angle?
        // println!("{:?}", axis);
        // println!("axis {:?} - force {:?}", axis, force);
        cue_stick_external_force.force = - (FORCE_CONVERSION_FACTOR * force).clamp(MIN_FORCE, MAX_FORCE) * Vec2::from_angle(angle);
        println!("{:?}", cue_stick_external_force.force);
        commands.insert_resource(NextState(Some(AppState::Game)));
        commands.insert_resource(NextState(Some(GameState::ShotCooldown)))
    }
}

pub fn compute_strike_force(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    cue_ball_query: Query<&Transform, (With<CueBall>, Without<CueStick>)>,
    mut cue_stick_query: Query<&mut Transform, With<CueStick>>,
    mut strike_force: ResMut<StrikeForce>,
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