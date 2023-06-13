use bevy::prelude::*;

// #[derive(Resource)]
// pub struct MouseWheelActive(pub bool);

// impl Default for MouseWheelActive {
//     fn default() -> Self {
//         MouseWheelActive(true)
//     }
// }


type Force = f32;

/// Keeps track of the total impulse that must be exerted on the Cue Stick.
#[derive(Resource)]
pub struct StrikeForce(pub Force);

impl Default for StrikeForce {
    fn default() -> Self {
        StrikeForce(0.0)        
    }
}

#[derive(Resource)]
pub struct CueStickLifetimeTimer {
    pub timer: Timer
}