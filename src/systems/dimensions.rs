use std::sync::Arc;

use feather_api::base::world::DimensionInfo;
use feather_api::common::world::{Dimensions, WorldPath};
use feather_api::prelude::*;
use feather_api::worldgen::{SuperflatWorldGenerator, VoidWorldGenerator, WorldGenerator};

pub fn register(app: &mut App) {
    app.add_startup_system_to_stage(StartupStage::PreStartup, add_vanilla_dimensions)
        .add_system_to_stage(CoreStage::PreUpdate, add_vanilla_dimensions);
}

fn add_vanilla_dimensions(
    biomes: Res<Arc<BiomeList>>,
    mut worlds: Query<(&WorldName, &WorldPath, &mut Dimensions), Added<Dimensions>>,
    config: Res<Config>,
) {
    if !worlds.is_empty() {
        for namespace in std::fs::read_dir("worldgen")
            .expect("There's no worldgen/ folder. Try removing generated/ and re-running feather")
            .flatten()
        {
            let dimension_namespace = namespace
                .file_name()
                .to_str()
                .expect("Non-UTF8 characters are not allowed in file names in worldgen/")
                .to_owned();
            let namespace_dir = namespace.path();
            for file in std::fs::read_dir(namespace_dir.join("dimension"))
                .iter_mut()
                .flatten()
                .flatten()
            {
                if let Some(dimension_value) = file
                    .file_name()
                    .to_str()
                    .expect("Non-UTF8 characters are not allowed in file names in worldgen/")
                    .strip_suffix(".json")
                {
                    let dimension_id = format!("{}:{}", dimension_namespace, dimension_value);
                    let mut dimension_info: DimensionInfo =
                        serde_json::from_str(&std::fs::read_to_string(file.path()).unwrap())
                            .expect(&format!(
                                "Invalid dimension format in file {:?}",
                                file.file_name()
                            ));

                    let (type_namespace, type_value) = dimension_info
                        .r#type
                        .split_once(':')
                        .expect("Invalid dimension type: should contain `:`.");
                    if type_value.contains(':') {
                        panic!("Invalid dimension type: should contain `:` exactly once.")
                    }
                    let type_path = format!(
                        "worldgen/{}/dimension_type/{}.json",
                        type_namespace, type_value
                    );
                    dimension_info.info =
                        serde_json::from_str(&std::fs::read_to_string(&type_path).unwrap())
                            .expect(&format!("Invalid dimension format in file {:?}", type_path));

                    for (world_name, world_path, mut dimensions) in worlds.iter_mut() {
                        if !dimensions.iter().any(|dim| *dim.id() == dimension_id) {
                            let generator: Arc<dyn WorldGenerator> =
                                match &config.worlds.generator[..] {
                                    "flat" => Arc::new(SuperflatWorldGenerator::from_config(
                                        &config.worlds.generator_settings,
                                    )),
                                    "void" => Arc::new(VoidWorldGenerator::from_config(
                                        &config.worlds.generator_settings,
                                    )),
                                    other => {
                                        log::error!(
                                            "Invalid generator specified in config.toml: {}",
                                            other
                                        );
                                        std::process::exit(1);
                                    }
                                };

                            log::info!(
                                "Adding dimension `{}` to world `{}`",
                                dimension_id,
                                **world_name
                            );
                            dimensions.add(Dimension::new(
                                dimension_id.clone(),
                                dimension_info.clone(),
                                generator,
                                world_path
                                    .join("dimensions")
                                    .join(type_namespace)
                                    .join(type_value),
                                biomes.clone(),
                            ));
                        }
                    }
                }
            }
        }
    }
}
