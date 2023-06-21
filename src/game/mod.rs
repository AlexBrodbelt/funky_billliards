use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier2d::prelude::*;

pub mod resources;
pub mod systems;
pub mod events;

pub mod ball;
pub mod cuestick;
pub mod pocket;
pub mod walls;
pub mod scoreboard;

use crate::{
    config::*,
    AppState
};

use systems::*;

use ball::BallPlugin;
use cuestick::CueStickPlugin;
use pocket::PocketPlugin;
use scoreboard::ScoreboardPlugin;
use walls::WallPlugin;

use self::resources::{TableStatus, ActivePlayer};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add our gameplay simulation systems to the fixed timestep schedule
        // Configure how frequently our gameplay systems are run

        app // States
            .add_state::<GameSetupState>()
            .add_state::<GameState>()
            .add_state::<SimulationState>()
            // Resources
            .insert_resource(TableStatus::default())
            .insert_resource(ActivePlayer::default())
            .insert_resource(FixedTime::new_from_secs(TIME_STEP))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            // Events
            .add_event::<CollisionEvent>()
            // On Update Systems
            .add_systems(
                (
                    toggle_simulation,
                    switch_player_condition,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameState::Playing))
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
            .add(GamePlugin)
            .add(BallPlugin)
            .add(CueStickPlugin)
            .add(PocketPlugin)
            .add(ScoreboardPlugin)  
            .add(WallPlugin)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    Paused,
    #[default]
    Running, 
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameSetupState {
    WallSetup,
    PocketSetup,
    CueBallSetup,
    ShotSetup,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    ShotCooldown,
    Playing,
    #[default]
    Disabled,
}

