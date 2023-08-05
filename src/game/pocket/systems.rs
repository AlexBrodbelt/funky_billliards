use bevy::{prelude::*, input::mouse::{MouseButtonInput}};
use bevy_rapier2d::prelude::*;

use super::components::*;

use crate::{game::{
    scoreboard::resources::*,
    resources::{ActivePlayer, WallStatus},
    ball::components::Ball, ui::pocket_set_up_menu::events::PocketSetUpEvent, GameSetUpState
    },
    resources::CursorPosition, config::POCKET_RADIUS, systems::{despawn, despawn_ref}
};





pub fn spawn_pocket(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(PocketBundle::new(Pocket::BottomLeft, meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::BotttomCenter, meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::BottomRight, meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::TopLeft, meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::TopCenter, meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::TopRight, meshes, materials ));
}

fn set_default_pockets(
    commands: &mut Commands,
    wall_status: &Res<WallStatus>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let wall_vertices =  &wall_status.vertex_buffer;

    // pockets on vertices
    for wall_vertex in wall_vertices {
        commands.spawn(PocketBundle::from_vec(*wall_vertex, meshes, materials));
    }
    // pockets in between vertices
    if let Some(index_buffer) = &wall_status.maybe_index_buffer {
        for [i,j] in index_buffer {
            let v1 = wall_vertices[*i as usize];
            let v2 = wall_vertices[*j as usize];
            if 1.5 * v1.distance(v2) > POCKET_RADIUS {
                commands.spawn(PocketBundle::from_vec(0.5 * (v1 + v2), meshes, materials));
            }
        }
    }
}

pub fn pocket_set_up_event_handler(
    mut commands: Commands,
    mut ev_pocketsetup: EventReader<PocketSetUpEvent>,
    pocket_query: Query<Entity, With<Pocket>>,
    cursor_position: Res<CursorPosition>,
    wall_status: Res<WallStatus>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_game_set_up_state: ResMut<NextState<GameSetUpState>>,
) {
    for ev in &mut ev_pocketsetup {
        match *ev {
            PocketSetUpEvent::SetPocket => {
                set_pocket(&mut commands, &mut meshes, &mut materials, &cursor_position)
            },
            PocketSetUpEvent::ClearPockets => {
                despawn_ref::<Pocket>(&mut commands, &pocket_query)
            },
            PocketSetUpEvent::Done => {
                if pocket_query.is_empty() {
                    println!("at least one pocket is needed")
                } else {
                    next_game_set_up_state.set(GameSetUpState::BallSetUp);
                }
            },
            PocketSetUpEvent::SetDefaultPockets => {
                set_default_pockets(&mut commands, &wall_status, &mut meshes, &mut materials)
            },
        }
    }
}

fn set_pocket(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    cursor_position: &Res<CursorPosition>,
    // mut cue_ball_query: Query<&mut Transform, With<CueBall>>,
    // mut pocket_status: ResMut<PocketStatus>,
    // mut next_cue_ball_state: ResMut<NextState<CueBallState>>,
) {
    commands.spawn(PocketBundle::from_vec(cursor_position.0, meshes, materials));    
}

pub fn pocket_condition(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    rapier_context: Res<RapierContext>,
    active_player: Res<ActivePlayer>,
    pocket_query: Query<Entity, With<Pocket>>,
    ball_query: Query<&Ball, With<Ball>>,
) {
    for pocket in pocket_query.iter() {
        for (entity1, entity2, intersecting) in rapier_context.intersections_with(pocket){
            if intersecting {

                // if Ok then entity1 is the ball entity otherwise entity2 is the ball entity, the pocket entity is the alternative entity option
                let (entity, ball_type) = match ball_query.get(entity1) {
                    Ok(ball_type) => (entity1, ball_type),
                    Err(_) => (entity2, ball_query.get(entity2).unwrap()),
                };

                scoreboard.pocket(&active_player.0, ball_type);
                commands.entity(entity).despawn(); 
                             
            }
        }
    }
}

// pub fn despawn_pockets( // idea make despawning generic function that takes in the component to despawn
//     mut commands: Commands,
//     pocket_query: Query<Entity, With<Pocket>>,
// ) {
//     for pocket in &pocket_query {
//         commands.entity(pocket).despawn();
//     }
// }

