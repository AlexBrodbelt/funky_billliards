use bevy::prelude::*;

use crate::{
    game::{
        ui::wall_set_up_menu::{
            components::WallSetUpMenuButton,
            styles::*
        },
        GameSetUpState, resources::WallStatus, walls::{components::Wall, systems::{clear_wall, spawn_default_walls}, WallSetUpState}},
    AppState
};


pub fn interact_with_button(
    mut commands: Commands,
    mut wall_query: Query<Entity, With<Wall>>,
    mut wall_status: ResMut<WallStatus>,
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor, &WallSetUpMenuButton),
    (Changed<Interaction>, With<WallSetUpMenuButton>)
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
    mut next_wall_set_up_state: ResMut<NextState<WallSetUpState>>,
) {
    if let Ok((interaction, mut background_color, pocket_set_up_menu_button_type)) = button_query.get_single_mut() {       
        match *pocket_set_up_menu_button_type {
            WallSetUpMenuButton::Clear => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_CLEAR_BUTTON_COLOR.into();
                        clear_wall(&mut commands, &mut wall_query, &mut wall_status);
                        next_wall_set_up_state.set(WallSetUpState::Edit);
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_CLEAR_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_CLEAR_BUTTON_COLOR.into();
                    },
                }
            },
            WallSetUpMenuButton::Done => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_DONE_BUTTON_COLOR.into();
                        if wall_status.vertex_buffer.len() > 2 {
                            next_game_setup_state.set(GameSetUpState::PocketSetUp);
                        } else {
                            println!("Wall is not well defined, three vertices are required to generate a well defined wall");
                        }
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_DONE_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_DONE_BUTTON_COLOR.into();
                    },
                }
            },
            WallSetUpMenuButton::Default => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_DEFAULT_BUTTON_COLOR.into();
                        next_wall_set_up_state.set(WallSetUpState::Disabled);
                        clear_wall(&mut commands, &mut wall_query, &mut wall_status);
                        spawn_default_walls(&mut commands, &mut wall_status);
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_DEFAULT_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_DEFAULT_BUTTON_COLOR.into();
                    },
                }
            },
        }
    }
}