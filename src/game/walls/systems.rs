use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;

use crate::{
    game::{
        resources::WallStatus,
        ui::wall_set_up_menu::events::WallSetUpEvent, 
        GameSetUpState
    },
    resources::CursorPosition
};

use super::components::*;


pub fn spawn_wall(
    commands: &mut Commands,
    vertices:Vec<Vec2>,
    indices: Option<Vec<[u32; 2]>> ,
) {
    println!("wall spawned");
    commands.spawn(WallBundle::new(vertices, indices));
}




pub fn wall_set_up_events_handler(
    mut commands: Commands,
    mut ev_wallsetup: EventReader<WallSetUpEvent>,
    wall_query: Query<Entity, With<Wall>>,
    cursor_position: Res<CursorPosition>,
    mut wall_status: ResMut<WallStatus>,
    mut next_game_setup_state: ResMut<NextState<GameSetUpState>>,    
) {
    for ev in ev_wallsetup.iter() {
        match ev {
            WallSetUpEvent::SetWallVertex => {
                set_wall_vertex(&mut commands, &wall_query, &cursor_position, &mut wall_status);
            },
            WallSetUpEvent::ClearWall => {
                clear_wall(&mut wall_status);
            },
            WallSetUpEvent::Done => {
                wall_set_up_done(&wall_status, &mut next_game_setup_state); 
            },
            WallSetUpEvent::SpawnDefaultWall => {
                spawn_default_wall(&mut commands, &mut wall_status);
            },
        }
    }
}

/// Helper function to [´set_wall_vertex()´]
pub fn build_wall_shape_bundle(vertex_buffer: &Vec<Vec2>) -> ShapeBundle {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(*vertex_buffer.first().unwrap());
        for  vertex in vertex_buffer.iter().rev() {
                path_builder.line_to(*vertex);
            }                
            
            let path = path_builder.build();

            ShapeBundle {
        path,
        transform: Transform::from_xyz(0., 75., 0.),
        ..default()
    }
}

/// Helper function to [´set_wall_vertex()´]
pub fn build_wall_collider(vertex_buffer: Vec<Vec2>, maybe_index_buffer: Option<Vec<[u32; 2]>>) -> Collider {
    Collider::polyline(vertex_buffer, maybe_index_buffer)
}

/// When in [`GameSetupState::WallSetup`] if the the cursor is pressed then a new vertex is added to the wall
fn set_wall_vertex(
    commands: &mut Commands, 
    wall_query: &Query<Entity, With<Wall>>,
    cursor_position: &Res<CursorPosition>,
    wall_status: &mut ResMut<WallStatus>,
) {
    // push new vertex into wall vertex buffer
    wall_status.vertex_buffer.push(cursor_position.0);
    let indices = 0..(wall_status.vertex_buffer.len() as u32);
    // generate index buffer - Vec<[0,1], [1,2], .., [n-1, n], [n, 0]>
    wall_status.maybe_index_buffer = Some(indices.circular_tuple_windows::<(_, _)>()
                                                                .map(|(i,j)| { [i, j] })
                                                                .collect::<Vec<[u32; 2]>>());
    if let Ok(wall_entity) = wall_query.get_single() {
        commands.entity(wall_entity)
            // .remove::<(Collider, ShapeBundle)>()
            .insert(build_wall_shape_bundle(&wall_status.vertex_buffer))
            .insert(build_wall_collider(wall_status.vertex_buffer.clone(), wall_status.maybe_index_buffer.clone()));
    } else {
        if (wall_status.vertex_buffer.len() >= 2) && wall_status.is_changed() {
            println!("vertex buffer: {:?}, index buffer: {:?}", wall_status.vertex_buffer, wall_status.maybe_index_buffer);
            spawn_wall(commands, wall_status.vertex_buffer.clone(), wall_status.maybe_index_buffer.clone());
        }
    }  
}

/// Clears the vertex and index buffer of the [`WallStatus`] resource 
fn clear_wall(
    wall_status: &mut ResMut<WallStatus>,
) {
    wall_status.clear_buffers();                         
}


fn wall_set_up_done(
    wall_status: &ResMut<WallStatus>,
    next_game_setup_state: &mut ResMut<NextState<GameSetUpState>>,
) {
    if wall_status.vertex_buffer.len() > 2 {
        next_game_setup_state.set(GameSetUpState::PocketSetUp);
    } else {
        println!("Wall is not well defined, three vertices are required to generate a well defined wall");
    }
}


fn spawn_default_wall(
    wall_status: &mut ResMut<WallStatus>, 
) {
    wall_status.set_to_default();
}


pub fn despawn_wall(
    mut commands: Commands,
    wall_query: Query<Entity, With<Wall>>,
    mut wall_status: ResMut<WallStatus>,
) {
    if let Ok(wall_entity) = wall_query.get_single() {
        commands.entity(wall_entity).despawn();
    }
    wall_status.clear_buffers();
}