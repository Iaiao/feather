//! Loads a [`Config`] from a TOML config.

use std::{fs, net::IpAddr, path::Path, str::FromStr};

use anyhow::Context;
use serde::{Deserialize, Deserializer};

use crate::core::Gamemode;
use crate::game::favicon::Favicon;
use crate::game::options::ServerOptions;

const DEFAULT_CONFIG: &str = include_str!("../../../../assets/default-config.toml");

/// Loads the config, creating a default config if needed.
pub fn load(path: &str) -> anyhow::Result<Config> {
    let path = Path::new(path);
    let default_config = DEFAULT_CONFIG;

    if !path.exists() {
        fs::write(path, default_config)?;
        log::info!("Created a default config");
    }

    let config_string = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_string).context("Invalid config.toml file")?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub network: Network,
    pub server: ServerConfig,
    pub log: Log,
    pub worlds: Worlds,
    pub proxy: Proxy,
}

impl Config {
    pub fn to_options(&self) -> ServerOptions {
        ServerOptions {
            port: self.network.port,
            bind_address: self.network.address.to_string(),
            favicon: Favicon::load_default(),
            motd: self.server.motd.clone(),
            online_mode: if self.proxy.proxy_mode != ProxyMode::None {
                false
            } else {
                self.server.online_mode
            },
            compression_threshold: if self.network.compression_threshold <= 0 {
                None
            } else {
                Some(self.network.compression_threshold as usize)
            },
            max_players: self.server.max_players,
            proxy_mode: match self.proxy.proxy_mode {
                ProxyMode::None => None,
                ProxyMode::Bungee => Some(crate::game::options::ProxyMode::Bungeecord),
                ProxyMode::Velocity => Some(crate::game::options::ProxyMode::Velocity),
            },
            velocity_secret: self.proxy.velocity_secret.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Network {
    pub address: IpAddr,
    pub port: u16,
    pub compression_threshold: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub online_mode: bool,
    pub motd: String,
    pub max_players: u32,
    pub default_gamemode: Gamemode,
    pub view_distance: u32,
    pub default_dimension: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Log {
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: log::LevelFilter,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Worlds {
    pub worlds: Vec<String>,
    pub default_world: String,
    pub generator: String,
    pub generator_settings: String,
    pub seed: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Proxy {
    pub proxy_mode: ProxyMode,
    pub velocity_secret: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProxyMode {
    None,
    Bungee,
    Velocity,
}

fn deserialize_log_level<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<log::LevelFilter, D::Error> {
    let string: String = String::deserialize(deserializer)?;
    let level = log::LevelFilter::from_str(&string).map_err(|_| {
        serde::de::Error::custom(
            "invalid log level: valid options are trace, debug, info, warn, error",
        )
    })?;
    Ok(level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let _config: Config = toml::from_str(DEFAULT_CONFIG).unwrap();
    }
}
