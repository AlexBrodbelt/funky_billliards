use bevy::prelude::*;

use crate::{
    game::{
        ui::{pocket_set_up_menu::{
            components::PocketSetUpMenuButton,
            styles::*, events::PocketSetUpEvent
        }, components::CanvasButton},
        GameSetUpState
    },
    AppState
};


pub fn interact_with_button(
    mut ev_pocketsetup: EventWriter<PocketSetUpEvent>,
    mut set: ParamSet<(
        Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CanvasButton>)
        >,
        Query<
        (&Interaction, &mut BackgroundColor, &PocketSetUpMenuButton),
        (Changed<Interaction>, With<PocketSetUpMenuButton>)
        >
    )>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
) {
    if let Ok((
        canvas_interaction,
        mut background_color
    )) = set.p0().get_single_mut() {
        match *canvas_interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_CLEAR_BUTTON_COLOR.into();
                ev_pocketsetup.send(PocketSetUpEvent::SetPocket);
            },
            Interaction::Hovered => {
                *background_color = HOVERED_CLEAR_BUTTON_COLOR.into();                        
            },
            Interaction::None => {
                *background_color = NORMAL_CLEAR_BUTTON_COLOR.into();
            },
        }
    }
    if let Ok((
        interaction, 
        mut background_color, 
        pocket_set_up_menu_button_type
    )) = set.p1().get_single_mut() {
        match *pocket_set_up_menu_button_type {
            PocketSetUpMenuButton::Clear => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_CLEAR_BUTTON_COLOR.into();
                        ev_pocketsetup.send(PocketSetUpEvent::ClearPockets);
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
                    Interaction::Pressed => {
                        *background_color = PRESSED_DONE_BUTTON_COLOR.into();
                        ev_pocketsetup.send(PocketSetUpEvent::Done);
                        // next_game_setup_state.set(GameSetUpState::BallSetUp)
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_DONE_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_DONE_BUTTON_COLOR.into();
                    },
                }
            },
            PocketSetUpMenuButton::Default => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_DONE_BUTTON_COLOR.into();
                        ev_pocketsetup.send(PocketSetUpEvent::SetDefaultPockets);
                    },
                    Interaction::Hovered => {
                        *background_color = HOVERED_DONE_BUTTON_COLOR.into();                        
                    },
                    Interaction::None => {
                        *background_color = NORMAL_DONE_BUTTON_COLOR.into();
                    },
                }
            }
        }
    }
}