use bevy::prelude::*;

use crate::{game::ui::{wall_set_up_menu::{
    components::{WallSetUpMenu, WallSetUpMenuButton},
    styles::*
}, styles::{CANVAS_BUTTON_STYLE, CANVAS_STYLE, TOOL_BAR_STYLE}, components::CanvasButton},
styles::BUTTON_STYLE, config::BACKGROUND_COLOR};


pub fn spawn_wall_set_up_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_wall_set_up_menu(&mut commands, &asset_server);
}

fn build_wall_set_up_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    commands
        // POCKET SET UP MENU
        .spawn(
            (
                NodeBundle {
                    style: POCKET_SET_UP_MENU_STYLE,
                    // background_color: Color::ANTIQUE_WHITE.into(), 
                    ..default()
                },
                WallSetUpMenu,
            )
        )
        // TOOL BAR
        .with_children(|pocket_set_up_menu| {
            pocket_set_up_menu.spawn((
                    NodeBundle {
                        style: TOOL_BAR_STYLE,
                        background_color: Color::ANTIQUE_WHITE.into(), 
                        ..default()
                    },
            ))
            // Done Button
            .with_children(|tool_bar| {
                // Done ✔ Button
                tool_bar.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_DONE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    WallSetUpMenuButton::Done,
                )
            )
                // Done Button Text 
                .with_children(|done_button| {
                    done_button.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Done",
                                        get_button_text_style(asset_server)
                                    )
                                ],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                });

                // Clear Button ✖
                tool_bar.spawn(
                    (
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_CLEAR_BUTTON_COLOR.into(),
                            ..default()
                        },
                        WallSetUpMenuButton::Clear,                
                    )
                )
                // Clear Button Text
                .with_children(|clear_button| {
                    clear_button.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Clear",
                                    get_button_text_style(asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                    );
                });

                // Default Button
                tool_bar.spawn(
                    (
                        ButtonBundle {
                            style: BUTTON_STYLE,
                            background_color: NORMAL_CLEAR_BUTTON_COLOR.into(),
                            ..default()
                        },
                        WallSetUpMenuButton::Default,                
                    )
                )
                // Default Button Text
                .with_children(|default_button| {
                    default_button.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Default",
                                    get_button_text_style(asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                    );
                });        
            });
            // CANVAS
            pocket_set_up_menu.spawn(
            (
                NodeBundle {
                    style: CANVAS_STYLE,
                    ..default()
                },
                CanvasButton,
            )
            )
            .with_children(|canvas| {
                canvas.spawn(
                    (
                        ButtonBundle {
                            style: CANVAS_BUTTON_STYLE,
                            background_color: BACKGROUND_COLOR.into(),
                            ..default()
                        },
                        CanvasButton,
                    )
                );
            });    
        });
}


pub fn despawn_wall_set_up_menu(
    mut commands: Commands,
    wall_set_up_menu_query: Query<Entity, With<WallSetUpMenu>>,
) {
    if let Ok(pause_menu_entity) = wall_set_up_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}