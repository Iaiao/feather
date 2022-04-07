use crate::game::favicon::Favicon;

/// Options for building a [`Server`](crate::Server).
#[derive(Debug, Clone)]
pub struct ServerOptions {
    /// Port to listen on.
    pub port: u16,
    /// Addresses to bind to.
    pub bind_address: String,

    /// The server favicon.
    pub favicon: Option<Favicon>,
    /// The server MOTD.
    pub motd: String,

    /// Whether the server should authenticate players.
    pub online_mode: bool,

    /// Maximum number of players to allow on the server.
    pub max_players: u32,

    /// Proxy IP forwarding mode
    pub proxy_mode: Option<ProxyMode>,
    // HMAC key used with Velocity IP forwarding.
    pub velocity_secret: String,

    /// Packet size threshold at which to compress data
    pub compression_threshold: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProxyMode {
    Bungeecord,
    Velocity,
}
