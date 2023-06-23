use bevy::prelude::*;

use crate::config::*;

pub const SCORE_POSITION_1_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    position: UiRect {
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..UiRect::DEFAULT
    },
    ..Style::DEFAULT
};

pub const SCORE_POSITION_2_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    position: UiRect {
    top: SCOREBOARD_TEXT_PADDING,
    right: SCOREBOARD_TEXT_PADDING,
    ..UiRect::DEFAULT
},
..Style::DEFAULT
};

