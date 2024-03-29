// use bevy::{ui::Style, a11y::accesskit::Size, prelude::Color};
use bevy::prelude::*;

// pub const NORMAL_BUTTON_COLOR: Color = Color::DARK_GRAY;
// pub const HOVERED_BUTTON_COLOR: Color = Color::GRAY;
// pub const PRESSED_BUTTON_COLOR: Color = Color::BLACK;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub const TITLE_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Px(300.0);
    style.height = Val::Px(120.0);
    style
};

pub const MENU_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style.row_gap = Val::Px(8.0);
    style.column_gap = Val::Px(8.0);
    style
};

// pub const _IMAGE_STYLE: Style = Style {
//     size: Size::new(Val::Px(200.0), Val::Px(64.0)),
//     margin: UiRect { 
//         left: Val::Px(8.0),
//         right: Val::Px(8.0),
//         top: Val::Px(8.0),
//         bottom: Val::Px(8.0),
//     }, 
//     ..default()
// };

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    }
}

pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
        ..default()
    }
}
