use bevy::prelude::*;

pub const POCKET_SET_UP_MENU_STYLE: Style = {
    let mut style: Style = Style::DEFAULT;
    style.display = Display::Flex;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.width = Val::Percent(100.0);
    style.height = Val::Percent(100.0);
    style
};

// pub const LHS_STYLE: Style = Style {
//     display: Display::Flex,
//     flex_direction: FlexDirection::Row,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     width: Val::Px(200.0),
//     height: Val::Percent(80.0),
//     margin: UiRect::new(Val::Px(32.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0)),
//     ..default()
// };

// pub const RHS_STYLE: Style = Style {
//     display: Display::Flex,
//     flex_direction: FlexDirection::Row,
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     size: Size::new(Val::Px(200.0), Val::Percent(80.0)),
//     margin: UiRect::new(Val::Px(0.0), Val::Px(32.0), Val::Px(0.0), Val::Px(0.0)),
//     ..default()
// };



pub const NORMAL_DONE_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_DONE_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_DONE_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub const NORMAL_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_CLEAR_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75,0.35);

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    }
}