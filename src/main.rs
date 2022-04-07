#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use std::sync::Arc;

extern crate feather_api;

use feather_api::bevy::app::{ScheduleRunnerSettings, StartupStage};
use feather_api::bevy::ecs::prelude::ParallelSystemDescriptorCoercion;
use feather_api::bevy::prelude::SystemStage;
use feather_api::bevy::MinimalPlugins;
use feather_api::game::config;
use feather_api::game::config::Config;
use feather_api::prelude::*;

mod packet_handlers;
mod systems;

const PLUGINS_DIRECTORY: &str = "plugins";
const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() {
    let config = config::load(CONFIG_PATH).expect("failed to load configuration file");

    feather_logging::init(config.log.level);
    log::info!("Loaded config");

    log::info!("Creating server");
    let options = config.to_options();
    let server = Server::bind(options).await.unwrap();
    let mut app = App::new();

    app.insert_resource(server)
        .insert_resource(ScheduleRunnerSettings::run_loop(TICK_DURATION))
        .add_plugins(MinimalPlugins)
        .insert_resource(config)
        .init_resource::<ChunkEntities>()
        .add_startup_stage_before(
            StartupStage::PreStartup,
            "_extract",
            SystemStage::single(data_generators::extract_vanilla_data),
        )
        .add_startup_stage_after("_extract", "_add_worlds", SystemStage::single(add_worlds))
        .add_startup_stage_after(
            "_add_worlds",
            "_init_biomes",
            SystemStage::single(init_biomes),
        );

    if let Ok(plugins) = std::fs::read_dir(PLUGINS_DIRECTORY) {
        for plugin in plugins.flatten() {
            if plugin.file_type().unwrap().is_file() {
                log::info!(
                    "Loading plugin: {}",
                    plugin
                        .file_name()
                        .to_str()
                        .expect("Invalid characters in plugin's file name")
                );
                unsafe {
                    app.load_plugin(plugin.path().to_str().unwrap());
                }
            }
        }
    }

    systems::register(&mut app);

    app.run();
}

fn add_worlds(mut commands: Commands, config: Res<Config>) {
    for world_name in config.worlds.worlds.iter() {
        let world = WorldBundle::new(world_name, format!("worlds/{}", world_name));
        commands.spawn_bundle(world);
    }
}

fn init_biomes(mut commands: Commands) {
    let mut biomes = BiomeList::default();

    for dir in std::fs::read_dir("worldgen/")
        .expect("There's no worldgen/ folder. Try removing generated/ and re-running feather")
        .flatten()
    {
        let biome_namespace = dir
            .file_name()
            .to_str()
            .expect("Non-UTF8 characters are not allowed in file names in worldgen/")
            .to_owned();
        for file in std::fs::read_dir(dir.path().join("worldgen/biome"))
            .iter_mut()
            .flatten()
            .flatten()
        {
            if let Some(biome_value) = file
                .file_name()
                .to_str()
                .expect("Non-UTF8 characters are not allowed in file names in worldgen/")
                .strip_suffix(".json")
            {
                let biome: BiomeGeneratorInfo =
                    serde_json::from_str(&std::fs::read_to_string(file.path()).unwrap()).unwrap();
                let biome_key = format!("{}:{}", biome_namespace, biome_value);
                log::trace!("Loaded biome: {}", biome_key);
                biomes.insert(biome_key, biome);
            }
        }
    }
    commands.insert_resource(Arc::new(biomes))
}
