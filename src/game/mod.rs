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
use scoreboard::ScoreboardPlugin;
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
            // On Update Systems
            .add_systems(
                (
                    toggle_simulation,
                )
                .in_set(OnUpdate(AppState::Game))
            )
            // On Update Systems when SimulationState::Running
            .add_systems(
                (
                    play_collision_sound,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),  
            );
    }
}


pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BallPlugin)
            // .add(CueStickPlugin)
            .add(PocketPlugin)
            .add(ScoreboardPlugin)  
            .add(WallPlugin)
            .add(GamePlugin)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
    
}