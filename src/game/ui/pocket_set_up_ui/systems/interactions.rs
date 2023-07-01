use bevy::prelude::*;

use crate::{
    game::{
        ui::pocket_set_up_ui::{
            components::PocketSetUpMenuButton,
            styles::*
        },
        GameSetUpState},
    AppState
};


pub fn interact_with_button(
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor, &PocketSetUpMenuButton),
    (Changed<Interaction>, With<PocketSetUpMenuButton>)
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
) {
    if let Ok((interaction, mut background_color, pocket_set_up_menu_button_type)) = button_query.get_single_mut() {
        match *pocket_set_up_menu_button_type {
            PocketSetUpMenuButton::Clear => {
                match *interaction {
                    Interaction::Clicked => {
                        *background_color = CLICKED_CLEAR_BUTTON_COLOR.into();
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_CLEAR_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_CLEAR_BUTTON_COLOR.into();
                    },
                }
            },
            PocketSetUpMenuButton::Done => {
                match *interaction {
                    Interaction::Clicked => {
                        *background_color = CLICKED_DONE_BUTTON_COLOR.into();
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_DONE_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_DONE_BUTTON_COLOR.into();
                    },
                }
            },
        }
    }
}