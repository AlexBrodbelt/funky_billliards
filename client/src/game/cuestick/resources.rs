use bevy::prelude::*;

// #[derive(Resource)]
// pub struct MouseWheelActive(pub bool);

// impl Default for MouseWheelActive {
//     fn default() -> Self {
//         MouseWheelActive(true)
//     }
// }


type Distance = f32;

/// Keeps track of the total impulse that must be exerted on the Cue Stick.
#[derive(Resource)]
pub struct WindUpDistance(pub Distance);

impl Default for WindUpDistance {
    fn default() -> Self {
        WindUpDistance(0.0)        
    }
}

