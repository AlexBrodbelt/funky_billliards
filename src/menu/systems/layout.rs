use bevy::{prelude::*, utils::petgraph::graph::Node};

use crate::menu::{components::{Menu, PlayButton, QuitButton}, styles::{NORMAL_BUTTON_STYLE, NORMAL_BUTTON_COLOR, get_button_text_style}};

pub fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);

}

pub fn despawn_menu (
    mut commands: Commands,
    menu_query: Query<Entity, With<Menu>>
) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}

fn build_main_menu (
    commands: &mut Commands, asset_server: &Res<AssetServer>
) -> Entity {
    let main_menu_entity = commands
        .spawn(
    (
                NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new( Val::Px(8.0), Val::Px(8.0)),
                    ..default()
                },
                background_color: Color::YELLOW_GREEN.into(),
                ..default()
                },
                Menu,
            )
        )
        .with_children(|parent| {
            // === Title ===
            parent.spawn(
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new( Val::Px(300.0), Val::Px(120.0) ),
                        ..default()
                    },
                    ..default()
                }
            )
            .with_children(|parent| {
                // Image 1
                // Text
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Funky Billiards",
                                    get_title_text_style(asset_server)
                                )
                            ],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
                // Image 2
            });

            // === Play Button ===
            parent.spawn(
                (
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton,
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Play",
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
            // === Quit Button ===
            parent.spawn(
                (
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton,
                )
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection::new(
                                    "Quit",
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
        })
        .id();

    main_menu_entity
}

// fn build_button(color: Color, button_type: Button) -> ButtonBundle {
//     ButtonBundle::default()
// }



