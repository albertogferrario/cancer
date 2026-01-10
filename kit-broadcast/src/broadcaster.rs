//! The main broadcaster for managing channels and sending messages.

use crate::channel::{ChannelInfo, ChannelType, PresenceMember};
use crate::message::{BroadcastMessage, ServerMessage};
use crate::Error;
use dashmap::DashMap;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// A connected client.
pub struct Client {
    /// Unique socket ID.
    pub socket_id: String,
    /// Sender to push messages to this client.
    pub sender: mpsc::Sender<ServerMessage>,
    /// Channels this client is subscribed to.
    pub channels: Vec<String>,
}

/// Shared state for the broadcaster.
struct BroadcasterInner {
    /// Connected clients by socket ID.
    clients: DashMap<String, Client>,
    /// Channels by name.
    channels: DashMap<String, ChannelInfo>,
    /// Optional authorization callback.
    authorizer: Option<Arc<dyn ChannelAuthorizer>>,
}

/// The broadcaster manages channels and client connections.
#[derive(Clone)]
pub struct Broadcaster {
    inner: Arc<BroadcasterInner>,
}

impl Broadcaster {
    /// Create a new broadcaster.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(BroadcasterInner {
                clients: DashMap::new(),
                channels: DashMap::new(),
                authorizer: None,
            }),
        }
    }

    /// Set the channel authorizer.
    pub fn with_authorizer<A: ChannelAuthorizer + 'static>(self, authorizer: A) -> Self {
        Self {
            inner: Arc::new(BroadcasterInner {
                clients: DashMap::new(),
                channels: DashMap::new(),
                authorizer: Some(Arc::new(authorizer)),
            }),
        }
    }

    /// Register a new client connection.
    pub fn add_client(&self, socket_id: String, sender: mpsc::Sender<ServerMessage>) {
        info!(socket_id = %socket_id, "Client connected");
        self.inner.clients.insert(
            socket_id.clone(),
            Client {
                socket_id,
                sender,
                channels: Vec::new(),
            },
        );
    }

    /// Remove a client and clean up their subscriptions.
    pub fn remove_client(&self, socket_id: &str) {
        if let Some((_, client)) = self.inner.clients.remove(socket_id) {
            info!(socket_id = %socket_id, "Client disconnected");

            // Remove from all subscribed channels
            for channel_name in &client.channels {
                self.unsubscribe_internal(socket_id, channel_name);
            }
        }
    }

    /// Subscribe a client to a channel.
    pub async fn subscribe(
        &self,
        socket_id: &str,
        channel_name: &str,
        auth: Option<&str>,
        member_info: Option<PresenceMember>,
    ) -> Result<(), Error> {
        let channel_type = ChannelType::from_name(channel_name);

        // Check authorization for private/presence channels
        if channel_type.requires_auth() {
            if let Some(authorizer) = &self.inner.authorizer {
                let auth_data = AuthData {
                    socket_id: socket_id.to_string(),
                    channel: channel_name.to_string(),
                    auth_token: auth.map(|s| s.to_string()),
                };
                if !authorizer.authorize(&auth_data).await {
                    warn!(socket_id = %socket_id, channel = %channel_name, "Authorization failed");
                    return Err(Error::unauthorized("Channel authorization failed"));
                }
            } else if auth.is_none() {
                return Err(Error::unauthorized("Authorization required"));
            }
        }

        // Get or create channel
        let mut channel = self
            .inner
            .channels
            .entry(channel_name.to_string())
            .or_insert_with(|| ChannelInfo::new(channel_name));

        // Add subscriber
        channel.add_subscriber(socket_id.to_string());

        // For presence channels, add member info
        if channel_type == ChannelType::Presence {
            if let Some(member) = member_info {
                channel.add_member(member.clone());

                // Notify other members
                let msg = ServerMessage::MemberAdded {
                    channel: channel_name.to_string(),
                    user_id: member.user_id.clone(),
                    user_info: member.user_info.clone(),
                };
                drop(channel); // Release lock before broadcasting
                self.send_to_channel_except(channel_name, socket_id, &msg)
                    .await;
            }
        } else {
            drop(channel);
        }

        // Update client's channel list
        if let Some(mut client) = self.inner.clients.get_mut(socket_id) {
            if !client.channels.contains(&channel_name.to_string()) {
                client.channels.push(channel_name.to_string());
            }
        }

        debug!(socket_id = %socket_id, channel = %channel_name, "Subscribed to channel");
        Ok(())
    }

    /// Unsubscribe a client from a channel.
    pub async fn unsubscribe(&self, socket_id: &str, channel_name: &str) {
        self.unsubscribe_internal(socket_id, channel_name);
    }

    fn unsubscribe_internal(&self, socket_id: &str, channel_name: &str) {
        // Remove from channel
        if let Some(mut channel) = self.inner.channels.get_mut(channel_name) {
            channel.remove_subscriber(socket_id);

            // For presence channels, notify about member leaving
            if channel.channel_type == ChannelType::Presence {
                if let Some(member) = channel.remove_member(socket_id) {
                    let msg = ServerMessage::MemberRemoved {
                        channel: channel_name.to_string(),
                        user_id: member.user_id,
                    };
                    // We can't await here, so we'll spawn a task
                    let channel_name = channel_name.to_string();
                    let broadcaster = self.clone();
                    tokio::spawn(async move {
                        broadcaster.send_to_channel(&channel_name, &msg).await;
                    });
                }
            }

            // Clean up empty channels
            if channel.is_empty() {
                drop(channel);
                self.inner.channels.remove(channel_name);
            }
        }

        // Update client's channel list
        if let Some(mut client) = self.inner.clients.get_mut(socket_id) {
            client.channels.retain(|c| c != channel_name);
        }

        debug!(socket_id = %socket_id, channel = %channel_name, "Unsubscribed from channel");
    }

    /// Broadcast a message to a channel.
    pub async fn broadcast<T: Serialize>(
        &self,
        channel: &str,
        event: &str,
        data: T,
    ) -> Result<(), Error> {
        let msg = BroadcastMessage::new(channel, event, data);
        let server_msg = ServerMessage::Event(msg);
        self.send_to_channel(channel, &server_msg).await;
        Ok(())
    }

    /// Broadcast to a channel, excluding a specific client.
    pub async fn broadcast_except<T: Serialize>(
        &self,
        channel: &str,
        event: &str,
        data: T,
        except_socket_id: &str,
    ) -> Result<(), Error> {
        let msg = BroadcastMessage::new(channel, event, data);
        let server_msg = ServerMessage::Event(msg);
        self.send_to_channel_except(channel, except_socket_id, &server_msg)
            .await;
        Ok(())
    }

    /// Send a message to all subscribers of a channel.
    async fn send_to_channel(&self, channel_name: &str, msg: &ServerMessage) {
        if let Some(channel) = self.inner.channels.get(channel_name) {
            for socket_id in channel.subscribers.iter() {
                self.send_to_client(socket_id, msg.clone()).await;
            }
        }
    }

    /// Send a message to all subscribers except one.
    async fn send_to_channel_except(
        &self,
        channel_name: &str,
        except_socket_id: &str,
        msg: &ServerMessage,
    ) {
        if let Some(channel) = self.inner.channels.get(channel_name) {
            for socket_id in channel.subscribers.iter() {
                if socket_id.as_str() != except_socket_id {
                    self.send_to_client(socket_id, msg.clone()).await;
                }
            }
        }
    }

    /// Send a message to a specific client.
    async fn send_to_client(&self, socket_id: &str, msg: ServerMessage) {
        if let Some(client) = self.inner.clients.get(socket_id) {
            if let Err(e) = client.sender.send(msg).await {
                warn!(socket_id = %socket_id, error = %e, "Failed to send message to client");
            }
        }
    }

    /// Get channel info.
    pub fn get_channel(&self, name: &str) -> Option<ChannelInfo> {
        self.inner.channels.get(name).map(|c| c.clone())
    }

    /// Get number of connected clients.
    pub fn client_count(&self) -> usize {
        self.inner.clients.len()
    }

    /// Get number of active channels.
    pub fn channel_count(&self) -> usize {
        self.inner.channels.len()
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Authorization data for private/presence channels.
#[derive(Debug, Clone)]
pub struct AuthData {
    /// The socket ID requesting access.
    pub socket_id: String,
    /// The channel name.
    pub channel: String,
    /// Optional auth token from the client.
    pub auth_token: Option<String>,
}

/// Trait for authorizing channel access.
#[async_trait::async_trait]
pub trait ChannelAuthorizer: Send + Sync {
    /// Check if access should be granted.
    async fn authorize(&self, data: &AuthData) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_broadcaster_basic() {
        let broadcaster = Broadcaster::new();
        let (tx, _rx) = mpsc::channel(32);

        broadcaster.add_client("socket_1".into(), tx);
        assert_eq!(broadcaster.client_count(), 1);

        broadcaster.remove_client("socket_1");
        assert_eq!(broadcaster.client_count(), 0);
    }

    #[tokio::test]
    async fn test_subscribe_public_channel() {
        let broadcaster = Broadcaster::new();
        let (tx, _rx) = mpsc::channel(32);

        broadcaster.add_client("socket_1".into(), tx);
        broadcaster
            .subscribe("socket_1", "orders", None, None)
            .await
            .unwrap();

        assert_eq!(broadcaster.channel_count(), 1);
        let channel = broadcaster.get_channel("orders").unwrap();
        assert_eq!(channel.subscriber_count(), 1);
    }

    #[tokio::test]
    async fn test_subscribe_private_requires_auth() {
        let broadcaster = Broadcaster::new();
        let (tx, _rx) = mpsc::channel(32);

        broadcaster.add_client("socket_1".into(), tx);
        let result = broadcaster
            .subscribe("socket_1", "private-orders.1", None, None)
            .await;

        assert!(result.is_err());
    }
}
