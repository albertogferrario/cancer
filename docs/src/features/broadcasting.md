# Broadcasting

Ferro provides a Laravel Echo-inspired WebSocket broadcasting system for real-time communication. Push updates to clients instantly through public, private, and presence channels.

## Configuration

### Environment Variables

Configure broadcasting in your `.env` file:

```env
# Optional limits (0 = unlimited)
BROADCAST_MAX_SUBSCRIBERS=100
BROADCAST_MAX_CHANNELS=50

# Connection settings (in seconds)
BROADCAST_HEARTBEAT_INTERVAL=30
BROADCAST_CLIENT_TIMEOUT=60

# Allow client-to-client messages
BROADCAST_ALLOW_CLIENT_EVENTS=true
```

### Bootstrap Setup

In `src/bootstrap.rs`, create the broadcaster:

```rust
use ferro::{App, Broadcaster, BroadcastConfig};
use std::sync::Arc;

pub async fn register() {
    // ... other setup ...

    // Create broadcaster with environment config
    let config = BroadcastConfig::from_env();
    let broadcaster = Arc::new(Broadcaster::with_config(config));

    // Store in app state for handlers to access
    App::set_broadcaster(broadcaster);
}
```

### Manual Configuration

```rust
use ferro::{Broadcaster, BroadcastConfig};
use std::time::Duration;

let config = BroadcastConfig::new()
    .max_subscribers_per_channel(100)
    .max_channels(50)
    .heartbeat_interval(Duration::from_secs(30))
    .client_timeout(Duration::from_secs(60))
    .allow_client_events(true);

let broadcaster = Broadcaster::with_config(config);
```

## Channel Types

Channels are determined by their name prefix:

| Type | Prefix | Authorization | Use Case |
|------|--------|---------------|----------|
| Public | none | No | Public updates (news feed) |
| Private | `private-` | Yes | User-specific data |
| Presence | `presence-` | Yes | Track online users |

### Examples

```rust
// Public channel - anyone can subscribe
"orders"
"notifications"

// Private channel - requires authorization
"private-orders.123"
"private-user.456"

// Presence channel - tracks members
"presence-chat.1"
"presence-room.gaming"
```

## Broadcasting Messages

### Basic Broadcast

```rust
use ferro::Broadcast;

// In a controller or service
let broadcast = Broadcast::new(broadcaster.clone());

broadcast
    .channel("orders.123")
    .event("OrderUpdated")
    .data(&order)
    .send()
    .await?;
```

### Excluding the Sender

When a client triggers an action, exclude them from the broadcast:

```rust
broadcast
    .channel("chat.1")
    .event("NewMessage")
    .data(&message)
    .except(&socket_id)  // Don't send to this client
    .send()
    .await?;
```

### Direct Broadcaster API

```rust
// Broadcast to all subscribers
broadcaster
    .broadcast("orders", "OrderCreated", &order)
    .await?;

// Broadcast excluding a client
broadcaster
    .broadcast_except("chat.1", "MessageSent", &msg, &sender_socket_id)
    .await?;
```

## Channel Authorization

Private and presence channels require authorization.

### Implementing an Authorizer

```rust
use ferro::{AuthData, ChannelAuthorizer, async_trait};

pub struct MyAuthorizer {
    // Database connection, etc.
}

#[async_trait]
impl ChannelAuthorizer for MyAuthorizer {
    async fn authorize(&self, data: &AuthData) -> bool {
        // data.socket_id - The client's socket ID
        // data.channel - The channel being accessed
        // data.auth_token - Optional auth token from client

        // Example: Parse user ID from channel name
        if data.channel.starts_with("private-orders.") {
            let order_id: i64 = data.channel
                .strip_prefix("private-orders.")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            // Verify user owns this order
            return self.user_owns_order(data.auth_token.as_deref(), order_id).await;
        }

        false
    }
}
```

### Registering the Authorizer

```rust
let authorizer = MyAuthorizer::new(db_pool);
let broadcaster = Broadcaster::new().with_authorizer(authorizer);
```

## Presence Channels

Presence channels track which users are online.

### Subscribing with Member Info

```rust
use ferro::PresenceMember;

let member = PresenceMember::new(socket_id, user_id)
    .with_info(serde_json::json!({
        "name": user.name,
        "avatar": user.avatar_url,
    }));

broadcaster
    .subscribe(&socket_id, "presence-chat.1", Some(&auth_token), Some(member))
    .await?;
```

### Member Events

When members join or leave, events are automatically broadcast:

```json
// Member joined
{
    "type": "member_added",
    "channel": "presence-chat.1",
    "user_id": "123",
    "user_info": {"name": "Alice", "avatar": "..."}
}

// Member left
{
    "type": "member_removed",
    "channel": "presence-chat.1",
    "user_id": "123"
}
```

### Getting Channel Members

```rust
if let Some(channel) = broadcaster.get_channel("presence-chat.1") {
    for member in channel.get_members() {
        println!("User {} is online", member.user_id);
    }
}
```

## Message Types

### Server to Client

```rust
pub enum ServerMessage {
    // Connection established
    Connected { socket_id: String },

    // Subscription confirmed
    Subscribed { channel: String },

    // Subscription failed
    SubscriptionError { channel: String, error: String },

    // Unsubscribed
    Unsubscribed { channel: String },

    // Broadcast event
    Event(BroadcastMessage),

    // Presence: member joined
    MemberAdded { channel: String, user_id: String, user_info: Value },

    // Presence: member left
    MemberRemoved { channel: String, user_id: String },

    // Keepalive response
    Pong,

    // Error
    Error { message: String },
}
```

### Client to Server

```rust
pub enum ClientMessage {
    // Subscribe to channel
    Subscribe { channel: String, auth: Option<String> },

    // Unsubscribe from channel
    Unsubscribe { channel: String },

    // Client event (whisper)
    Whisper { channel: String, event: String, data: Value },

    // Keepalive
    Ping,
}
```

## WebSocket Handler Example

```rust
use ferro::{Broadcaster, ClientMessage, ServerMessage};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

async fn handle_websocket(
    ws_stream: WebSocketStream,
    broadcaster: Arc<Broadcaster>,
) {
    let socket_id = generate_socket_id();
    let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);

    // Register client
    broadcaster.add_client(socket_id.clone(), tx);

    // Send connection confirmation
    let connected = ServerMessage::Connected {
        socket_id: socket_id.clone()
    };
    // ... send to client

    // Handle messages
    loop {
        tokio::select! {
            // Message from client
            Some(msg) = ws_stream.next() => {
                match parse_client_message(msg) {
                    ClientMessage::Subscribe { channel, auth } => {
                        match broadcaster.subscribe(&socket_id, &channel, auth.as_deref(), None).await {
                            Ok(_) => { /* send Subscribed */ }
                            Err(e) => { /* send SubscriptionError */ }
                        }
                    }
                    ClientMessage::Unsubscribe { channel } => {
                        broadcaster.unsubscribe(&socket_id, &channel).await;
                    }
                    ClientMessage::Ping => {
                        // send Pong
                    }
                    _ => {}
                }
            }

            // Message to send to client
            Some(msg) = rx.recv() => {
                // Send to WebSocket
            }
        }
    }

    // Cleanup on disconnect
    broadcaster.remove_client(&socket_id);
}
```

## Example: Real-time Chat

```rust
// When a message is sent
async fn send_message(
    broadcaster: Arc<Broadcaster>,
    room_id: i64,
    user: &User,
    content: String,
    socket_id: &str,
) -> Result<Message, Error> {
    // Save to database
    let message = Message::create(room_id, user.id, &content).await?;

    // Broadcast to room (except sender)
    Broadcast::new(broadcaster)
        .channel(&format!("presence-chat.{}", room_id))
        .event("NewMessage")
        .data(serde_json::json!({
            "id": message.id,
            "user": {
                "id": user.id,
                "name": user.name,
            },
            "content": content,
            "created_at": message.created_at,
        }))
        .except(socket_id)
        .send()
        .await?;

    Ok(message)
}
```

## Monitoring

```rust
// Get connection stats
let client_count = broadcaster.client_count();
let channel_count = broadcaster.channel_count();

// Get specific channel info
if let Some(channel) = broadcaster.get_channel("orders") {
    println!("Subscribers: {}", channel.subscriber_count());
}
```

## Environment Variables Reference

| Variable | Description | Default |
|----------|-------------|---------|
| `BROADCAST_MAX_SUBSCRIBERS` | Max subscribers per channel (0=unlimited) | 0 |
| `BROADCAST_MAX_CHANNELS` | Max total channels (0=unlimited) | 0 |
| `BROADCAST_HEARTBEAT_INTERVAL` | Heartbeat interval (seconds) | 30 |
| `BROADCAST_CLIENT_TIMEOUT` | Client timeout (seconds) | 60 |
| `BROADCAST_ALLOW_CLIENT_EVENTS` | Allow whisper messages | true |

## Best Practices

1. **Use meaningful channel names** - `orders.{id}` not `channel1`
2. **Authorize private data** - Always use private channels for user-specific data
3. **Use presence for online status** - Track who's viewing/editing
4. **Exclude senders when appropriate** - Avoid echo effects
5. **Set reasonable limits** - Prevent resource exhaustion with config limits
6. **Handle disconnects gracefully** - Clean up subscriptions on disconnect
