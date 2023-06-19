// use bevy::{ui::Style, a11y::accesskit::Size, prelude::Color};
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::DARK_GRAY;

pub const NORMAL_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new( Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    size: Size::new(Val::Px(200.0), Val::Px(64.0)),
    margin: UiRect { 
        left: Val::Px(8.0),
        right: Val::Px(8.0),
        top: Val::Px(8.0),
        bottom: Val::Px(8.0),
    }, 
    ..Style::DEFAULT
};

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
