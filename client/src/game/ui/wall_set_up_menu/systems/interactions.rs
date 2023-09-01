use bevy::prelude::*;

use crate::{
    game::{
        ui::{wall_set_up_menu::{
            components::WallSetUpMenuButton,
            styles::*, events::WallSetUpEvent
        }, components::CanvasButton},
        GameSetUpState, 
        resources::WallStatus, 
        walls::WallSetUpState},
    AppState,
    config::{BACKGROUND_COLOR, PRESSED_CANVAS, HOVERED_CANVAS, NORMAL_CANVAS}
};


pub fn interact_with_button(
    mut ev_wallsetup: EventWriter<WallSetUpEvent>,
    // mut wall_query: Query<Entity, With<Wall>>,
    mut wall_status: ResMut<WallStatus>,
    mut set: ParamSet<
    (
        Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CanvasButton>)
        >,
        Query<
        (&Interaction, &mut BackgroundColor, &WallSetUpMenuButton),
        (Changed<Interaction>, With<WallSetUpMenuButton>)
        >
    )
    >,
    mut _next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
    mut next_wall_set_up_state: ResMut<NextState<WallSetUpState>>,
) {
    // Canvas button
    if let Ok((
            interaction,
            mut background_color
    )) = set.p0().get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_CANVAS.into(); // this is hacky and ugly fix this!!!
                ev_wallsetup.send(WallSetUpEvent::SetWallVertex);
            },
            Interaction::Hovered => {
                *background_color = HOVERED_CANVAS.into();
            },
            Interaction::None => {
                *background_color = NORMAL_CANVAS.into();
            },
        }
    }
    // Tool bar buttons
    if let Ok((
        interaction, 
        mut background_color, 
        pocket_set_up_menu_button_type
    )) = set.p1().get_single_mut() {       
        match *pocket_set_up_menu_button_type {
            WallSetUpMenuButton::Clear => {
                match *interaction {
                    Interaction::Pressed => {
                        *background_color = PRESSED_CLEAR_BUTTON_COLOR.into();
                        ev_wallsetup.send(WallSetUpEvent::ClearWall);
                        // next_wall_set_up_state.set(WallSetUpState::Edit);
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
                        ev_wallsetup.send(WallSetUpEvent::Done)
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
                        ev_wallsetup.send(WallSetUpEvent::SpawnDefaultWall);
                        next_wall_set_up_state.set(WallSetUpState::Disabled);

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