use bevy::prelude::*;

pub const POCKET_SET_UP_MENU_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(15.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new( Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const LEFT_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const MIDDLE_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    margin: UiRect::new(Val::Px(100.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const RIGHT_STYLE: Style = Style {
    display: Display::Flex,
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
    margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
    ..Style::DEFAULT
};

pub const DONE_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new( Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const CLEAR_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new( Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const DEFAULT_BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new( Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const NORMAL_DONE_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_DONE_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CLICKED_DONE_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub const NORMAL_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CLICKED_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub const NORMAL_DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CLICKED_DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    }
}