use bevy::prelude::*;

use crate::config::*;

pub const SCORE_POSITION_1_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.position_type = PositionType::Absolute;
    style.top = SCOREBOARD_TEXT_PADDING;
    style.left = SCOREBOARD_TEXT_PADDING;
    style
};

pub const SCORE_POSITION_2_STYLE: Style = {
    let mut style = Style::DEFAULT;
    
    style
};

