use bevy::{prelude::*};

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

// Defines the friction coefficient for the material of the table
pub const FRICTION_COEFFICIENT: f32 = 0.2;
pub const STOPPING_THRESHOLD: f32 = 100.0;

// Table dimensions for the rectangular table
pub const TABLE_HEIGHT: f32 = 600.0;
pub const TABLE_WIDTH: f32 = 1200.0;
// These pub constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
pub const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
pub const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
pub const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
pub const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
pub const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

pub const WALL_THICKNESS: f32 = 10.0;
// x coordinates
pub const LEFT_WALL: f32 = - TABLE_WIDTH / 2.0;
pub const RIGHT_WALL: f32 = TABLE_WIDTH / 2.0;
// y coordinates
pub const BOTTOM_WALL: f32 = - TABLE_HEIGHT / 2.0;
pub const TOP_WALL: f32 = TABLE_HEIGHT / 2.0;

// snooker table features
pub const X_BAULK_LINE : f32 = TABLE_WIDTH / 5.0; // x coordinate for Baulk Line
pub const R_BAULK_D : f32 = TABLE_HEIGHT / 6.0; // radius of Baulk D
pub const X_BAULK_D : f32 = X_BAULK_LINE - R_BAULK_D;
pub const Y_BAULK_D : f32 = TABLE_HEIGHT / 3.0;

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
pub const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;
pub const GAP_BETWEEN_BALLS: f32 = 0.5;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub const BACKGROUND_COLOR: Color = Color::rgb(0.04, 0.42, 0.01);
pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
pub const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
pub const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
pub const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);



// pub const BLACK: Color =  Color::rgb(0.0, 0.0, 0.0);
// pub const CUE_BALL: Color =  Color::rgb(255.0, 255.0, 255.0);
// pub const YELLOW: Color =  Color::rgb(255.0, 0.68, 0.26);
// pub const RED: Color =  Color::rgb(255.0, 0.0, 0.0);
// pub const BLUE: Color =  Color::rgb(0.0, 0.0, 255.0);
// pub const GREEN: Color =  Color::rgb(0.0, 255.0, 0.0);
// pub const PINK: Color =  Color::rgb(255.0, 0.08, 0.58);
// pub const BROWN: Color =  Color::rgb(0.55, 0.27, 0.07);
