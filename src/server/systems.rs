use bevy::prelude::*;
use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    RenetServer, ServerEvent,
};
use bevy_renet::transport::NetcodeServerPlugin;
use local_ip_address::local_ip;
use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use crate::{
    client::components::{ClientChannel, PlayerTransform},
    player::{
        components::{AnimationIndices, ControlledPlayer, Player},
        configs::PLAYER_RUNNING_ANIMATION_PATH,
    },
    setup::configs::{connection_config, PROTOCOL_ID, WINDOW_BOTTOM_Y, WINDOW_LEFT_X},
};

use super::components::{NetworkedEntities, ServerChannel, ServerLobby, ServerMessages};

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(connection_config());

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), 42069);
    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    println!("Running on local address: {}", inbound_server_addr);

    let current_time: std::time::Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![inbound_server_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
}

pub fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut lobby: ResMut<ServerLobby>,
    mut server: ResMut<RenetServer>,
    players: Query<(Entity, &Player, &Transform)>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);

                // Initialize other players for this new client
                for (entity, player, transform) in &players {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                // Spawn new player
                let transform = Transform {
                    translation: Vec3::new(WINDOW_LEFT_X + 100.0, WINDOW_BOTTOM_Y + 300.0, 0.0),
                    scale: Vec3::new(5., 5., 1.),
                    ..default()
                };
                let player_texture_handle = asset_server.load(PLAYER_RUNNING_ANIMATION_PATH);
                let player_texture_atlas = TextureAtlas::from_grid(
                    player_texture_handle,
                    Vec2::new(19.0, 32.0),
                    7,
                    1,
                    None,
                    None,
                );
                let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
                let player_animation_indices = AnimationIndices { first: 0, last: 6 };

                let player_entity = commands
                    .spawn((
                        SpriteSheetBundle {
                            texture_atlas: player_texture_atlas_handle,
                            transform,
                            ..default()
                        },
                        Player { id: *client_id },
                        player_animation_indices,
                    ))
                    .id();

                lobby.players.insert(*client_id, player_entity);

                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *client_id,
                    entity: player_entity,
                    translation,
                })
                .unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }

                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();
                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }
    }

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            let input: PlayerTransform = bincode::deserialize(&message).unwrap();
            if let Some(player_entity) = lobby.players.get(&client_id) {
                commands.entity(*player_entity).insert(Transform {
                    translation: Vec3::new(
                        input.translation[0],
                        input.translation[1],
                        input.translation[2],
                    ),
                    scale: Vec3::new(5., 5., 1.),
                    ..default()
                });
            }
        }
    }
}

pub fn server_network_sync(
    mut server: ResMut<RenetServer>,
    query_players: Query<(Entity, &GlobalTransform), Or<(With<Player>, With<ControlledPlayer>)>>,
) {
    let mut networked_entities = NetworkedEntities::default();
    for (entity, transform) in &query_players {
        networked_entities.entities.push(entity);
        networked_entities
            .translations
            .push(transform.translation().into());
    }

    let sync_message = bincode::serialize(&networked_entities).unwrap();
    server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
}
