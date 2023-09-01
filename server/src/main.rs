use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use renet_visualizer::RenetServerVisualizer;
use store::{connection_config, PROTOCOL_ID};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u64, Entity>,
}

fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());

    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addr,
    };

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    (server, transport)
}

#[allow(clippy::too_many_arguments)]
fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut netcode_server: ResMut<NetcodeServerTransport>,
    mut server_lobby: ResMut<ServerLobby>,
    mut visualizer: ResMut<RenetServerVisualizer<200>>,
    time: Res<Time>,
    mut egui_contexts: EguiContexts,
) {

    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id }=> {
                println!("Client {} connected", client_id);
                visualizer.add_client(*client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason} => {
                println!("Client disconnected: {}", client_id);
                if let Some(entity) = server_lobby.players.remove(&client_id) {
                    egui_contexts.remove(entity);
                }
            }
        }
    }

    for event in netcode_server_events {
        match event {
            ServerEvent::ClientConnected(client_id) => {
                println!("Client connected: {}", client_id);
            }
            ServerEvent::ClientDisconnected(client_id) => {
                println!("Client disconnected: {}", client_id);
                if let Some(entity) = server_lobby.players.remove(&client_id) {
                    egui_contexts.remove(entity);
                }
            }
            ServerEvent::ClientMessage(client_id, message) => {
                println!("Client message: {} {:?}", client_id, message);
            }
        }
    }

    netcode_server_visualizer.update(&server, &netcode_server, time.delta_seconds());
}

fn main() {    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(EguiPlugin)
        .run();
}