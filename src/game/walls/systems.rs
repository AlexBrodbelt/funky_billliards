use bevy::{prelude::*, input::mouse::MouseButtonInput};
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::{Itertools, EitherOrBoth::*};

use crate::{config::{WALL_COLOR, WALL_THICKNESS, PLAY_FIELD_COLOR}, game::resources::TableStatus, resources::CursorPosition};

use super::components::*;

pub fn spawn_default_walls(
    mut commands: Commands,
    // mut _meshes: ResMut<Assets<Mesh>>,
    // mut _materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(WallBundle::default());
}

pub fn build_wall(
    mut commands: Commands,
) {
    let path_builder = PathBuilder::new();
    let path = path_builder.build();

    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0., 75., 0.),
            ..default()
        },
        Stroke::new(WALL_COLOR, WALL_THICKNESS),
        Fill::color(PLAY_FIELD_COLOR),
    ));
}

/// When in [`GameSetupState::WallSetup`] if the the cursor is pressed then a new vertex is added to the wall
pub fn add_wall_segment(
    mut mouse_button_input: EventReader<MouseButtonInput>,
    mut wall_query: Query<(&mut Collider ,&mut Path), With<Wall>>,
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
    if let Ok(wall_entity) = wall_query.get_single_mut() {
        commands.entity(wall_entity).despawn();
    }
    table_status.clear_wall_buffers();
}

// pub fn spawn_walls(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
// commands.spawn(
//         ShapeBundle {
//             path,
//             transform: Transform::default(),
//             ..default()
//         }
//     );
// }

pub fn despawn_walls(
    mut commands: Commands,
    walls_query: Query<Entity, With<Wall>>,
) {
    for wall in &walls_query {
        commands.entity(wall).despawn();
    }
}



