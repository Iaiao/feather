use itertools::Itertools;

use crate::blocks::BlockId;

use crate::base::chunk::Chunk;
use crate::base::world::Sections;
use crate::prelude::CHUNK_WIDTH;
use crate::prelude::*;
use crate::worldgen::WorldGenerator;
use crate::BiomeList;

pub struct SuperflatWorldGenerator {
    pub options: SuperflatGeneratorOptions,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuperflatGeneratorOptions {
    pub layers: Vec<(BlockId, usize)>,
    pub biome: String,
}

impl WorldGenerator for SuperflatWorldGenerator {
    fn generate_chunk(
        &self,
        position: ChunkPosition,
        sections: Sections,
        min_y: i32,
        biomes: &BiomeList,
    ) -> Chunk {
        let biome = biomes
            .get_id(&self.options.biome)
            .unwrap_or_else(|| panic!("Biome does not exist: `{}`", self.options.biome));
        let mut chunk = Chunk::new(position, sections, min_y / 16);
        chunk
            .sections_mut()
            .iter_mut()
            .for_each(|s| s.biomes_mut().fill(biome));

        let mut y_counter = 0;
        for (block, height) in self.options.clone().layers {
            for y in y_counter..(y_counter + height as i32) {
                for x in 0..CHUNK_WIDTH {
                    for z in 0..CHUNK_WIDTH {
                        chunk
                            .set_block_at(x as usize, y as usize, z as usize, block, false)
                            .unwrap();
                    }
                }
            }

            y_counter += height as i32;
        }

        chunk.recalculate_heightmaps();

        chunk
    }

    fn is_flat(&self) -> bool {
        true
    }

    fn from_config(config: &str) -> Self {
        if let Some((layers, biome)) = config.split(';').collect_tuple() {
            SuperflatWorldGenerator {
                options: SuperflatGeneratorOptions {
                    layers: layers
                        .split(',')
                        .map(|layer| {
                            if let Some((block, count)) = layer.split_once('*') {
                                (
                                    block_from_str(block)
                                        .expect("Unknown block in generator_settings layer"),
                                    count.parse().unwrap_or(1),
                                )
                            } else {
                                (
                                    block_from_str(layer)
                                        .expect("Unknown block in generator_settings layer"),
                                    1,
                                )
                            }
                        })
                        .collect(),
                    biome: add_namespace_if_not_specified(biome),
                },
            }
        } else {
            panic!("Invalid generator_settings for superflat generator: `;` must occur in generator_settings exactly once. Example: \"bedrock*1,dirt*2,grass_block*1;plains\"")
        }
    }
}

fn block_from_str(s: &str) -> Option<BlockId> {
    BlockId::from_vanilla_id(
        BlockKind::from_namespaced_id(&add_namespace_if_not_specified(s))?.default_state_id(),
    )
}

fn add_namespace_if_not_specified(s: &str) -> String {
    match s.chars().filter(|c| *c == ':').count() {
        0 => format!("minecraft:{}", s),
        1 => s.to_string(),
        other => panic!(
            "`{}` is not a valid: `:` should occur 0 or 1 times, found occurrences: {}",
            s, other
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::blocks::BlockId;

    use crate::base::chunk::SECTION_HEIGHT;

    use super::*;

    #[test]
    pub fn test_worldgen_flat() {
        let options = SuperflatGeneratorOptions {
            biome: "minecraft:mountains".to_string(),
            layers: vec![
                (BlockId::bedrock(), 1),
                (BlockId::dirt(), 2),
                (BlockId::grass_block(), 1),
            ],
        };

        let chunk_pos = ChunkPosition { x: 1, z: 2 };
        let generator = SuperflatWorldGenerator { options };
        let mut biomes = BiomeList::default();
        biomes.insert(
            "minecraft:mountains".to_string(),
            BiomeGeneratorInfo {
                carvers: Default::default(),
                features: vec![],
                spawners: BiomeSpawners {
                    monster: vec![],
                    creature: vec![],
                    ambient: vec![],
                    axolotls: vec![],
                    underground_water_creature: vec![],
                    water_creature: vec![],
                    water_ambient: vec![],
                    misc: vec![],
                },
                spawn_costs: Default::default(),
                info: BiomeInfo {
                    effects: BiomeEffects {
                        mood_sound: None,
                        music: None,
                        ambient_sound: None,
                        additions_sound: None,
                        grass_color_modifier: None,
                        sky_color: BiomeColor { r: 0, g: 0, b: 0 },
                        foliage_color: None,
                        grass_color: None,
                        fog_color: BiomeColor { r: 0, g: 0, b: 0 },
                        water_color: BiomeColor { r: 0, g: 0, b: 0 },
                        water_fog_color: BiomeColor { r: 0, g: 0, b: 0 },
                    },
                    precipitation: "".to_string(),
                    temperature: 0.0,
                    downfall: 0.0,
                    temperature_modifier: None,
                    category: BiomeCategory::Mountain,
                    particle: None,
                },
            },
        );
        let chunk = generator.generate_chunk(chunk_pos, Sections(16), 0, &biomes);

        assert_eq!(chunk.position(), chunk_pos);
        for x in 0usize..16 {
            for z in 0usize..16 {
                for (y, block) in &[
                    (0usize, BlockId::bedrock()),
                    (1usize, BlockId::dirt()),
                    (2usize, BlockId::dirt()),
                    (3usize, BlockId::grass_block()),
                ] {
                    assert_eq!(chunk.block_at(x, *y, z).unwrap(), *block);
                }
                for y in 4..16 * SECTION_HEIGHT {
                    assert_eq!(
                        chunk.block_at(x as usize, y as usize, z as usize).unwrap(),
                        BlockId::air()
                    );
                }
                assert_eq!(
                    chunk.biome_at(x, 0, z).unwrap(),
                    biomes.get_id("minecraft:mountains").unwrap()
                );
            }
        }
    }

    #[test]
    pub fn test_config() {
        let config = "bedrock*1,dirt*2,minecraft:grass_block;plains";
        assert_eq!(
            SuperflatWorldGenerator::from_config(config).options,
            SuperflatGeneratorOptions {
                biome: "minecraft:plains".to_string(),
                layers: vec![
                    (BlockId::bedrock(), 1),
                    (BlockId::dirt(), 2),
                    (BlockId::grass_block(), 1)
                ],
            }
        )
    }
}
