use bevy::prelude::*;

use crate::game::ui::pocket_set_up_menu::{
    components::{PocketSetUpMenu, PocketSetUpMenuButton},
    styles::*
};


pub fn spawn_pocket_set_up_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_pocket_set_up_menu(&mut commands, &asset_server);
}

fn build_pocket_set_up_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>, 
) {
    let pocket_set_up_menu = commands
        .spawn(
            (
                NodeBundle {
                    style: POCKET_SET_UP_MENU_STYLE,
                    background_color: Color::ANTIQUE_WHITE.into(), 
                    ..default()
                },
                PocketSetUpMenu,
            )
        )
        .with_children(|parent| {
            // Done ✔ Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: DONE_BUTTON_STYLE,
                        background_color: NORMAL_DONE_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PocketSetUpMenuButton::Done,
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Done ✔",
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
            parent.spawn(
                (
                    ButtonBundle {
                        style: CLEAR_BUTTON_STYLE,
                        background_color: NORMAL_CLEAR_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PocketSetUpMenuButton::Clear,                
                )
            )
            .with_children(|parent| {
                parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Clear ✖",
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
}

pub fn despawn_pocket_set_up_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PocketSetUpMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}