use bevy::prelude::*;
use bevy_renet::renet::{ChannelConfig, ClientId, SendType};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

#[derive(Component, Serialize, Deserialize)]
pub struct PlayerTransform {
    pub translation: [f32; 3],
}

#[derive(Default, Resource)]
pub struct NetworkMapping(pub HashMap<Entity, Entity>);

#[derive(Debug)]
pub struct PlayerInfo {
    pub client_entity: Entity,
    pub server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    pub players: HashMap<ClientId, PlayerInfo>,
}

#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connected;

pub enum ClientChannel {
    Input,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Input => 0,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::Input.into(),
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::ZERO,
            },
        }]
    }
}
