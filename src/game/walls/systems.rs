use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;

use crate::{
    game::resources::WallStatus,
    resources::CursorPosition
};

use super::components::*;

pub fn spawn_default_walls(
    commands: &mut Commands,
    wall_status: &mut ResMut<WallStatus>,
    // mut _meshes: ResMut<Assets<Mesh>>,
    // mut _materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    // commands.spawn(WallBundle::default());
    wall_status.set_to_default();
}

pub fn spawn_wall(
    commands: &mut Commands,
    vertices:Vec<Vec2>,
    indices: Option<Vec<[u32; 2]>> ,
) {
    println!("wall spawned");
    commands.spawn(WallBundle::new(vertices, indices));
}

// fn build_path(vertex_buffer: &Vec<Vec2>, maybe_index_buffer: &Option<Vec<[u32; 2]>>) -> Path {
    

// }

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

pub fn build_wall_collider(vertex_buffer: Vec<Vec2>, maybe_index_buffer: Option<Vec<[u32; 2]>>) -> Collider {
    Collider::polyline(vertex_buffer, maybe_index_buffer)
}

/// When in [`GameSetupState::WallSetup`] if the the cursor is pressed then a new vertex is added to the wall
pub fn set_wall_vertex(
    mut commands: Commands, 
    mut mouse_button_input: EventReader<MouseButtonInput>,
    wall_query: Query<Entity, With<Wall>>,
    cursor_position: Res<CursorPosition>,
    mut wall_status: ResMut<WallStatus>,
) {
    if let Some(_button_pressed) = mouse_button_input.iter().last() {
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
                spawn_wall(&mut commands, wall_status.vertex_buffer.clone(), wall_status.maybe_index_buffer.clone());
            }
        }
    }
}

// pub fn update_wall_entity(
//     mut commands: Commands,
//     mut wall_query: Query<(&mut Collider, &mut Path), With<Wall>>,
//     wall_status: Res<WallStatus>,
// ) {
//     if let Ok((mut wall_collider, mut wall_path)) = wall_query.get_single_mut() {
//         let mut wall_path_builder = PathBuilder::new();
//         for (i, vertex) in wall_status.vertex_buffer.iter().enumerate() {
//             if i == 0 {
//                 wall_path_builder.move_to(*vertex);
//             } else {
//                 wall_path_builder.line_to(*vertex);
//             }                
//         }
//         wall_path_builder.close(); // connects the current position with the starting position closing the shape
//         // update the Path component
//         *wall_path = wall_path_builder.build();
//         // update the Collider component
//         *wall_collider = Collider::polyline(
//             wall_status.vertex_buffer.clone(),
//             wall_status.maybe_index_buffer.clone()
//         );
//     } else {
//         if (wall_status.vertex_buffer.len() >= 2) && wall_status.is_changed() {
//             println!("vertex buffer: {:?}, index buffer: {:?}", wall_status.vertex_buffer, wall_status.maybe_index_buffer);
//             spawn_wall(&mut commands, wall_status.vertex_buffer.clone(), wall_status.maybe_index_buffer.clone());
//         }
//     }
// }

/// Clears the vertex and index buffer of the [`WallStatus`] resource 
pub fn clear_wall(
    wall_status: &mut ResMut<WallStatus>,
) {
    wall_status.clear_buffers();
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