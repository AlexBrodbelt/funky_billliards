use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;

use crate::{config::{WALL_COLOR, WALL_THICKNESS, PLAY_FIELD_COLOR}, game::resources::{TableStatus, WallStatus}, resources::CursorPosition};

use super::components::*;

pub fn spawn_default_walls(
    commands: &mut Commands,
    table_status: &mut ResMut<TableStatus>,
    // mut _meshes: ResMut<Assets<Mesh>>,
    // mut _materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(WallBundle::default());
    table_status.wall_status = WallStatus::default();
}

pub fn spawn_wall(
    mut commands: Commands,
) {
    println!("wall spawned");
    
    commands.spawn(WallBundle::new());
}

/// When in [`GameSetupState::WallSetup`] if the the cursor is pressed then a new vertex is added to the wall
pub fn add_wall_segment(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut wall_query: Query<(&mut Collider, &mut Path), With<Wall>>,
    cursor_position: Res<CursorPosition>,
    mut table_status: ResMut<TableStatus>,
) {
    if let Some(_button_pressed) = mouse_button_input.iter().last() {
        if let Ok((mut wall_collider, mut wall_path)) = wall_query.get_single_mut() {
            // push new vertex into wall vertex buffer
            table_status.wall_status.vertex_buffer.push(cursor_position.0);
            let indices = 0..(table_status.wall_status.vertex_buffer.len() as u32);
            // generate index buffer - Vec<[0,1], [1,2], .., [n-1, n], [n, 0]>
            // table_status.wall_status.index_buffer = indices.clone()
            //                                                 .into_iter()
            //                                                 .zip_longest(indices.skip(1)
            //                                                                     .into_iter()
            //                                                 )
            //                                                 .map(|both_or_left| -> [u32; 2] {
            //                                                     match both_or_left {
            //                                                         Both(i , j) => [i as u32, j as u32],
            //                                                         Left(i)     => [i as u32, 0],
            //                                                         Right(j)    => [j as u32, 0],
            //                                                     }
            //                                                 })
            //                                                 .collect::<Vec<[u32; 2]>>();
            table_status.wall_status.index_buffer = indices.circular_tuple_windows::<(_, _)>()
                                                            .map(|(i,j)| { [i, j] })
                                                            .collect::<Vec<[u32; 2]>>();
            let mut wall_path_builder = PathBuilder::new();
            for (i, vertex) in table_status.wall_status.vertex_buffer.iter().enumerate() {
                if i == 0 {
                    wall_path_builder.move_to(*vertex);
                } else {
                    wall_path_builder.line_to(*vertex);
                }                
            }
            wall_path_builder.close(); // connects the current position with the starting position closing the shape
            // update the Path component
            *wall_path = wall_path_builder.build();
            // update the Collider component
            *wall_collider = Collider::polyline(
                table_status.wall_status.vertex_buffer.clone(),
                Some(table_status.wall_status.index_buffer.clone())
            ); 
        } 
    }
}

pub fn clear_wall(
    commands: &mut Commands,
    wall_query: &mut Query<Entity, With<Wall>>,
    table_status: &mut ResMut<TableStatus>,
) {
    if let Ok(wall_entity) = wall_query.get_single() {
        commands.entity(wall_entity).despawn();
    }
    table_status.clear_wall_buffers();

    commands.spawn(WallBundle::new());
}

pub fn despawn_walls(
    mut commands: Commands,
    wall_query: Query<Entity, With<Wall>>,
) {
    if let Ok(wall) = wall_query.get_single() {
        commands.entity(wall).despawn();
    } 
}



