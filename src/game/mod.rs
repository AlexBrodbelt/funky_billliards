use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier2d::prelude::*;

pub mod resources;
pub mod systems;
pub mod events;

pub mod ball;
pub mod cue_stick;
pub mod pocket;
pub mod walls;
pub mod scoreboard;

use crate::{
    config::*,
    AppState
};

use self::systems::*;

use ball::BallPlugin;
use pocket::PocketPlugin;
use scoreboard::ScoreBoardPlugin;
use walls::WallPlugin;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add our gameplay simulation systems to the fixed timestep schedule
        // Configure how frequently our gameplay systems are run

        
        app.add_state::<SimulationState>()
            // Resources
            .insert_resource(FixedTime::new_from_secs(TIME_STEP))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            // Events
            .add_event::<CollisionEvent>()
            // Systems

            .add_system(play_collision_sound)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(bevy::window::close_on_esc);
    }
}


pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BallPlugin)
            // .add(CueStickPlugin)
            .add(PocketPlugin)
            .add(ScoreBoardPlugin)  
            .add(WallPlugin)
            .add(GamePlugin)         
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    Runnning,
    #[default]
    Paused,
}