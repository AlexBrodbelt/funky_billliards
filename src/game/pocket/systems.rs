use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use super::components::*;

use crate::{game::{
    scoreboard::resources::*,
    resources::{ActivePlayer, WallStatus},
    ball::components::Ball, ui::pocket_set_up_menu::events::PocketSetUpEvent, GameSetUpState
    },
    resources::CursorPosition, config::POCKET_RADIUS, systems::despawn_ref
};





pub fn spawn_pocket(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>, 
) {
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::BottomLeft), meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::BotttomCenter), meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::BottomRight), meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::TopLeft), meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::TopCenter), meshes, materials ));
    commands.spawn(PocketBundle::new(Pocket::Standard(StandardPocket::TopRight), meshes, materials ));
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
        commands.spawn(PocketBundle::new(
            Pocket::HandPlaced(PocketIdentifier { 
                id: 0,
                position: *wall_vertex
            }), 
            meshes, 
            materials
        ));

    }
    // pockets in between vertices
    if let Some(index_buffer) = &wall_status.maybe_index_buffer {
        for [i,j] in index_buffer {
            let v1 = wall_vertices[*i as usize];
            let v2 = wall_vertices[*j as usize];
            
            if 1.5 * v1.distance(v2) > POCKET_RADIUS {
                commands.spawn(PocketBundle::new(
                    Pocket::HandPlaced(PocketIdentifier { 
                        id: 0,
                        position: 0.5 * (v1 + v2),
                    }), 
                    meshes, 
                    materials
                ));
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
    commands.spawn(PocketBundle::new(
        Pocket::HandPlaced(PocketIdentifier {
            id: 0,
            position: cursor_position.0 
            }),
            meshes,
            materials));    
}


struct NoBallPocketedError;

fn maybe_ball_pocketed<'a>(contact: &Contact, ball_query: &'a Query<(Entity, &Ball), With<Ball>>, pocket_query: &Query<Entity, With<Pocket>>) -> Result<(Entity, &'a Ball), NoBallPocketedError> {
    match ball_query.get(contact.entity1) {
        Ok((entity, ball_type)) => {
            if let Ok((_)) = pocket_query.get(contact.entity2) {
                Ok((entity, ball_type))
            } else {
                Err(NoBallPocketedError)
            }
        },
        Err(_) => match ball_query.get(contact.entity2) {
            Ok((entity, ball_type)) => {
                if let Ok((_)) = pocket_query.get(contact.entity1) {
                    Ok((entity, ball_type))
                } else {
                    Err(NoBallPocketedError)
                }
            },
            Err(_) => Err(NoBallPocketedError),
        }
    }
}

pub fn pocket_condition(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut collision_event: EventReader<Collision>,
    active_player: Res<ActivePlayer>,
    pocket_query: Query<Entity, With<Pocket>>,
    ball_query: Query<(Entity, &Ball), With<Ball>>,
) {
    for ev in collision_event.iter() {
        let contact = ev.0;
        
        match maybe_ball_pocketed(&contact, &ball_query, &pocket_query) {
            Ok((entity, ball_type)) => {  
                // let ball_type = ball_query.get(entity).unwrap();
                println!("{:?} has been despawned", ball_type);
                scoreboard.pocket(&active_player.0, ball_type);
                commands.entity(entity).despawn();               
            },
            Err(_) => continue,
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

