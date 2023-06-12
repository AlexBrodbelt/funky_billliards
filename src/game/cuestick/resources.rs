use bevy::prelude::*;

// #[derive(Resource)]
// pub struct MouseWheelActive(pub bool);

// impl Default for MouseWheelActive {
//     fn default() -> Self {
//         MouseWheelActive(true)
//     }
// }


type Impulse = f32;

/// Keeps track of the total impulse that must be exerted on the Cue Stick.
#[derive(Resource)]
pub struct StrikeForce(pub Impulse);

impl Default for StrikeForce {
    fn default() -> Self {
        StrikeForce(0.0)        
    }
}