use bevy::prelude::*;

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

// Defines the friction coefficient for the material of the table
pub const FRICTION_COEFFICIENT: f32 = 0.3;
// pub const STOPPING_THRESHOLD: f32 = 100.0;

// Ball
pub const BALL_RADIUS: f32 = TABLE_WIDTH / 136.0;

// Pocket
pub const POCKET_COLOR: Color = Color::BLACK;
pub const POCKET_RADIUS: f32 = 20.0;

// Cuestick
pub const CUESTICK_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const CUESTICK_SIZE: Vec2 = Vec2::new(120.0, 5.0);
pub const MAX_FORCE: f32 = 5000.0;
pub const MIN_FORCE: f32 = 0.0;
pub const PULL_BACK_DISPLACEMENT_CONVERSION_FACTOR: f32 = 5.0;

// Gaps
pub const GAP_BETWEEN_BALLS: f32 = 8.0;
pub const GAP_BETWEEN_POCKET_AND_WALL: f32 = 25.0;
pub const GAP_BETWEEN_CUESTICK_AND_CUEBALL: f32 = 5.0;

// Table
pub const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.42, 0.01);
pub const TABLE_HEIGHT: f32 = 600.0;
pub const TABLE_WIDTH: f32 = 1200.0;
// These pub constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.

// Wall
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = - TABLE_WIDTH / 2.0;
pub const RIGHT_WALL: f32 = TABLE_WIDTH / 2.0;
// y coordinates
pub const BOTTOM_WALL: f32 = - TABLE_HEIGHT / 2.0;
pub const TOP_WALL: f32 = TABLE_HEIGHT / 2.0;

// Snooker table features
pub const X_BAULK_LINE : f32 = LEFT_WALL + (TABLE_WIDTH / 5.0); // x coordinate for Baulk Line
pub const R_BAULK_D : f32 = TABLE_HEIGHT / 6.0; // radius of Baulk D
pub const X_BAULK_D : f32 = X_BAULK_LINE - R_BAULK_D;
// pub const Y_BAULK_D : f32 = TABLE_HEIGHT / 3.0;

// Scoreboard
pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);


pub const FORCE_CONVERSION_FACTOR: f32 = 1500.0;


