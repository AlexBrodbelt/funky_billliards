use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier2d::prelude::*;

pub mod resources;
pub mod systems;
pub mod events;

pub mod ball;
pub mod cuestick;
pub mod pocket;
pub mod scoreboard;
pub mod ui;
pub mod walls;

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

use self::{resources::{ActivePlayer, PocketStatus, CueBallStatus, WallStatus, CueStickStatus}, ui::GameUIPlugin};


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Add our gameplay simulation systems to the fixed timestep schedule
        // Configure how frequently our gameplay systems are run

        app // States
            .add_state::<GameSetUpState>()
            .add_state::<GameState>()
            .add_state::<SimulationState>()
            // Resources
            .insert_resource(WallStatus::default())
            .insert_resource(PocketStatus::default())
            .insert_resource(CueBallStatus::default())
            .insert_resource(CueStickStatus::default())
            .insert_resource(ActivePlayer::default())
            .insert_resource(FixedTime::new_from_secs(TIME_STEP))
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            // Events
            .add_event::<CollisionEvent>()
            // On Update Systems
            .add_systems(
                Update,
                (
                    toggle_simulation,
                    stopping_threshold,
                    switch_player_condition,
                    play_collision_sound.run_if(in_state(SimulationState::Running)),
                )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Playing))
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
            .add(GameUIPlugin)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
    Paused,
    #[default]
    Running, 
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameSetUpState {
    WallSetUp,
    PocketSetUp,
    BallSetUp,
    CueBallSetUp,
    ShotSetUp,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    ShotCoolDown,
    Playing,
    #[default]
    Disabled,
}

