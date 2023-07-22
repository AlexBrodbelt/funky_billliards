use bevy::{prelude::*, app::AppExit};

use crate::{main_menu::{styles::{CLICKED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR}, components::MenuButton}, AppState, game::{GameSetUpState, walls::WallSetUpState}};

pub fn interact_with_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
    (&Interaction, &mut BackgroundColor, &MenuButton),
    (Changed<Interaction>, With<MenuButton>)
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,
    mut next_wall_set_up_state: ResMut<NextState<WallSetUpState>>,
) {
    if let Ok((interaction, mut background_color, menu_button_type)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                match *menu_button_type {
                    MenuButton::Play => {
                        next_app_state.set(AppState::GameSetUp);
                        next_game_setup_state.set(GameSetUpState::WallSetUp);
                        next_wall_set_up_state.set(WallSetUpState::Edit);
                    },
                    MenuButton::Quit => {
                        app_exit_event_writer.send(AppExit);
                    },
                }
            },
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            },
        }
    }
}

