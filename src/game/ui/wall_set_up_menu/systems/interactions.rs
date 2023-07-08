use bevy::prelude::*;

use crate::{
    game::{
        ui::wall_set_up_menu::{
            components::WallSetUpMenuButton,
            styles::*
        },
        GameSetUpState, resources::TableStatus, walls::{components::Wall, systems::{clear_wall, spawn_default_walls}}},
    AppState
};


pub fn interact_with_button(
    mut commands: Commands,
    mut wall_query: Query<Entity, With<Wall>>,
    mut table_status: ResMut<TableStatus>,
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor, &WallSetUpMenuButton),
    (Changed<Interaction>, With<WallSetUpMenuButton>)
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
) {
    if let Ok((interaction, mut background_color, pocket_set_up_menu_button_type)) = button_query.get_single_mut() {       
        match *pocket_set_up_menu_button_type {
            WallSetUpMenuButton::Clear => {
                match *interaction {
                    Interaction::Clicked => {
                        *background_color = CLICKED_CLEAR_BUTTON_COLOR.into();
                        clear_wall(&mut commands, &mut wall_query, &mut table_status);
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
                    Interaction::Clicked => {
                        *background_color = CLICKED_DONE_BUTTON_COLOR.into();
                        if table_status.wall_status.vertex_buffer.len() > 2 {
                            next_game_setup_state.set(GameSetUpState::PocketSetup);
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
                    Interaction::Clicked => {
                        *background_color = CLICKED_DEFAULT_BUTTON_COLOR.into();
                        // spawn_default_walls(&mut commands);
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