use bevy::prelude::*;

use crate::AppState;

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        commands.insert_resource(NextState(Some(AppState::Game)));
        println!("Entered {:?}", AppState::Game);
    }
}

pub fn transition_to_menu_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        commands.insert_resource(NextState(Some(AppState::Menu)));
        println!("Entered {:?}", AppState::Game);
    }    
}