use bevy::prelude::*;

#[derive(Event)]
pub enum PocketSetUpEvent {
    SetPocket,
    ClearPockets,
    Done,
    SetDefaultPockets
}