use super::block::BlockType;
use super::ChunkPosition;
use std::collections::HashMap;

/// The number of bits used for each block
/// in the global palette.
const GLOBAL_BITS_PER_BLOCK: u8 = 14;

/// The minimum bits per block allowed when
/// using a section palette.
/// Bits per block values lower than this
/// value will be offsetted to this value.
const MIN_BITS_PER_BLOCK: u8 = 4;

/// The maximum number of bits per block
/// allowed when using a section pallette.
/// Values above this will use the global pallette
/// instead.
const MAX_BITS_PER_BLOCK: u8 = 8;

/// A chunk column consisting
/// of a 16x256x16 section of blocks.
#[derive(Clone)]
pub struct Chunk {
    location: ChunkPosition,
    sections: [ChunkSection; 16],
    // TODO block entities
}

impl Default for Chunk {
    fn default() -> Self {
        // Rust apparently forces you to implement
        // `Copy` on types if you want to use the
        // `[ChunkSection::new(); 16]` syntax,
        // so I had to do this.
        let sections = [
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
            ChunkSection::new(),
        ];
        Self {
            location: ChunkPosition::new(0, 0),
            sections,
        }
    }
}

impl Chunk {
    /// Creates a new empty chunk
    /// with the specified location.
    pub fn new(location: ChunkPosition) -> Self {
        Self {
            location,
            ..Default::default()
        }
    }

    /// Gets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn block_at(&self, x: u16, y: u16, z: u16) -> BlockType {
        assert!(x < 16);
        assert!(y < 256);
        assert!(z < 16);
        let chunk_section = &self.sections[(y / 16) as usize];
        chunk_section.block_at(x, y % 16, z)
    }

    /// Sets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn set_block_at(&mut self, x: u16, y: u16, z: u16, block: BlockType) {
        assert!(x < 16);
        assert!(y < 256);
        assert!(z < 16);
        let chunk_section = &mut self.sections[(y / 16) as usize];
        chunk_section.set_block_at(x, y % 16, z, block);
    }

    /// Returns a slice of the 16
    /// chunk sections in the chunk.
    pub fn sections(&self) -> &[ChunkSection] {
        &self.sections
    }

    pub fn position(&self) -> ChunkPosition {
        self.location
    }
}

/// A chunk section consisting of
/// 16x16x16 blocks. A chunk section
/// maintains an array of `u64` which
/// are used with the global palette
/// or a section palette to store
/// block state information.
#[derive(Clone)]
pub struct ChunkSection {
    /// If true, this chunk section
    /// consists only of air and will
    /// not be sent in the Chunk Data packet.
    empty: bool,
    /// The number of bits used
    /// for each block in the
    /// data vector.
    bits_per_block: u8,
    /// The palette used for this
    /// chunk section.
    palette: Palette,
    /// Array of longs representing
    /// block state data, with each
    /// length of bits `bits_per_block`
    /// in length representing a single block
    /// state.
    data: Vec<u64>,
    /// The number of time each block type
    /// appears in this chunk section. This
    /// is used to determine when to resize
    /// the palette.
    occurrence_map: HashMap<BlockType, u16>,
    /// The number of distinct block types
    /// needed in the chunk section before
    /// the bits per block value must be increased.
    upper_threshold: u16,
    /// The number of distinct block types needed in
    /// the chunk section before the bits per
    /// block value should be decreased.
    lower_threshold: u16,
    /// Block light at each position in the chunk
    /// section, where 0 is darkest and 15 is brightest.
    /// Each block takes up half a byte in this array.
    block_light: [u8; 16 * 16 * 16 / 2],
    /// Sky light at each position in the chunk section,
    /// where 0 is darkest and 15 is brightest.
    /// Each block takes up half a byte in this array.
    sky_light: [u8; 16 * 16 * 16 / 2],
}

impl Default for ChunkSection {
    fn default() -> Self {
        Self::new()
    }
}

impl ChunkSection {
    /// Creates a new, empty chunk section.
    pub fn new() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert(0, 0);
        let mut occurrence_map = HashMap::new();
        occurrence_map.insert(BlockType::Air, 16 * 16 * 16);
        Self {
            empty: true,
            bits_per_block: 4,
            palette: Palette {
                global: false,
                palette: vec![0],
                mappings,
            },
            data: vec![0; (4 * 16 * 16 * 16) / 64],
            occurrence_map,
            upper_threshold: 32,
            lower_threshold: 1,
            block_light: [15; 2048],
            sky_light: [15; 2048],
        }
    }

    /// Returns the block at the given
    /// position, local to this chunk section.
    pub fn block_at(&self, x: u16, y: u16, z: u16) -> BlockType {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);

        let bit_index =
            (get_block_index_from_coords(x, y, z) as u32) * (self.bits_per_block as u32);

        let start_long_index = (bit_index / 64) as usize;
        let end_long_index = ((bit_index + (self.bits_per_block as u32) - 1) / 64) as usize;

        let start_long = self.data[start_long_index];
        let end_long = self.data[end_long_index];

        let index_in_long = bit_index % 64;

        let mut result = 0;

        let mask = ((1 << self.bits_per_block) - 1) as u64;

        result |= ((start_long >> index_in_long) & mask) as u16;

        if start_long_index != end_long_index {
            // Value stretches across multiple entries
            // in the data array
            let end_offset = 64 - index_in_long;
            result |= (end_long << end_offset) as u16;
        }

        self.palette.get_type_from_index(result)
    }

    /// Sets the block type at the given position,
    /// resizing the internal arrays as necessary.
    /// Calling this function could incur significant
    /// overhead if resizing is necessary.
    pub fn set_block_at(&mut self, x: u16, y: u16, z: u16, block: BlockType) {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);

        if block != BlockType::Air {
            self.empty = false;
        }

        let old = self.block_at(x, y, z);

        if self.occurrence_map.contains_key(&old) {
            let new_amnt = self.occurrence_map[&old] - 1;
            if new_amnt == 0 {
                self.occurrence_map.remove(&old);
                self.palette.remove_block_mapping(old);
                self.resize_bits_per_block(self.occurrence_map.len() as u16); // Inefficient to recalculate the entire thing if a block type no longer exists - TODO perhaps better implementation?
            } else {
                self.occurrence_map.insert(old, new_amnt);
            }
        }

        let amnt = self.occurrence_map.get(&block).cloned();
        if let Some(amnt) = amnt {
            self.occurrence_map.insert(block, amnt + 1);
        } else {
            // New block
            self.update_palette(block);
        }

        let bit_index =
            (get_block_index_from_coords(x, y, z) as u32) * (self.bits_per_block as u32);

        let start_long_index = (bit_index / 64) as usize;
        let end_long_index = ((bit_index + (self.bits_per_block as u32) - 1) / 64) as usize;
        let index_in_long = (bit_index % 64) as u64;

        let paletted_id = self.palette.get_index_from_type(block) as u64;

        self.data[start_long_index] |= (paletted_id << index_in_long) as u64;

        if start_long_index != end_long_index {
            self.data[end_long_index] |= (paletted_id >> (64 - index_in_long)) as u64;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn data(&self) -> &[u64] {
        &self.data
    }

    pub fn bits_per_block(&self) -> u8 {
        self.bits_per_block
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    /// Updates the palette to account for the new
    /// block type and recalculates the data array
    /// if necessary.
    fn update_palette(&mut self, new_block_type: BlockType) {
        if self.palette.global {
            return;
        }

        self.palette.add_block_mapping(new_block_type);

        let new_block_count = (self.occurrence_map.len() + 1) as u16;
        if new_block_count >= self.upper_threshold || new_block_count < self.lower_threshold {
            self.resize_bits_per_block(new_block_count);
        }
    }

    fn resize_bits_per_block(&mut self, new_block_count: u16) {
        // Resize
        self.bits_per_block = bits_per_block_needed(new_block_count as u16);

        if self.bits_per_block > MAX_BITS_PER_BLOCK {
            self.palette.global = true;
            self.bits_per_block = GLOBAL_BITS_PER_BLOCK;
        } else if self.bits_per_block < MIN_BITS_PER_BLOCK {
            self.bits_per_block = MIN_BITS_PER_BLOCK;
        }

        self.new_thresholds();

        self.recalculate();
    }

    fn new_thresholds(&mut self) {
        self.upper_threshold = 2u16.pow((self.bits_per_block + 1) as u32);
        self.lower_threshold = 2u16.pow((self.bits_per_block - 1) as u32);
    }

    /// Recalculates the data field
    /// based on the current palette.
    fn recalculate(&mut self) {
        // TODO
    }
}

fn bits_per_block_needed(block_count: u16) -> u8 {
    // The number of bits per block needed
    // is the floored base-2 logarithm of
    // the number of blocks in the chunk section
    ((block_count as f32).log(2.0)) as u8
}

/// Returns the index into the data and light
/// arrays for the given coordinates (local to
/// a chunk section).
fn get_block_index_from_coords(x: u16, y: u16, z: u16) -> u16 {
    (x + (z * 16)) + (y * (16 * 16))
}

/*
/// The inverse of `get_block_index_from_coords`.
/// The returned tuple is in the order (x, y, z).
fn get_coords_from_block_index(index: u16) -> (u16, u16, u16) {
    let x;
    let y;
    let z;

    y = index / 256;
    x = (index % 256) % 16;
    z = (index % 256) / 16;

    (x, y, z)
}
*/

/// A section palette as used by
/// `ChunkSection`.
#[derive(Clone)]
pub struct Palette {
    /// If set to `true`, the global
    /// palette is used rather than this
    /// palette.
    global: bool,
    /// The palette, mapping indices
    /// of this array to block state IDs
    /// in the global palette.
    palette: Vec<u16>,
    /// Mapping of global state IDs
    /// to indices in this palette. This
    /// value is used to accelerate performance.
    mappings: HashMap<u16, u16>,
}

impl Palette {
    /// Updates the palette to allow
    /// for a new block type to be added
    /// in.
    fn add_block_mapping(&mut self, block_type: BlockType) {
        if !self.global {
            let global_id = block_type.block_state_id();
            self.palette.push(global_id);
            self.mappings
                .insert(global_id, (self.palette.len() - 1) as u16);
        }
    }

    fn remove_block_mapping(&mut self, block_type: BlockType) {
        if !self.global {
            let global_id = block_type.block_state_id();
            self.palette.retain(|val| *val != global_id);
            self.mappings.remove(&global_id).unwrap();
        }
    }

    /// Returns the block type corresponding to
    /// the index into this palette or from the global
    /// palette.
    fn get_type_from_index(&self, index: u16) -> BlockType {
        if self.global {
            BlockType::from_block_state_id(index)
        } else {
            BlockType::from_block_state_id(self.palette[index as usize])
        }
    }

    /// Returns the mapping for the specified
    /// block type.
    fn get_index_from_type(&self, block: BlockType) -> u16 {
        if self.global {
            block.block_state_id()
        } else {
            self.mappings[&block.block_state_id()]
        }
    }

    pub fn data(&self) -> &[u16] {
        &self.palette
    }

    pub fn global(&self) -> bool {
        self.global
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_block_index_from_coords() {
        assert_eq!(get_block_index_from_coords(0, 1, 0), 256);
        assert_eq!(get_block_index_from_coords(1, 1, 1), 256 + 16 + 1);
    }

    /*#[test]
    fn test_get_coords_from_block_index() {
        assert_eq!(get_coords_from_block_index(256), (0, 1, 0));
        assert_eq!(get_coords_from_block_index(256 + 16 + 1), (1, 1, 1));
    }*/

    #[test]
    fn test_chunk_section() {
        let mut section = ChunkSection::new();
        section.set_block_at(0, 0, 0, BlockType::Granite);
        section.set_block_at(0, 15, 0, BlockType::Andesite);
        section.set_block_at(4, 4, 4, BlockType::Stone);

        assert_eq!(section.block_at(0, 0, 0), BlockType::Granite);
        assert_eq!(section.block_at(0, 15, 0), BlockType::Andesite);
        assert_eq!(section.block_at(4, 4, 4), BlockType::Stone);
    }

    #[test]
    fn test_chunk() {
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        chunk.set_block_at(0, 0, 0, BlockType::Andesite);
        chunk.set_block_at(0, 14, 0, BlockType::PolishedAndesite);
        chunk.set_block_at(5, 12, 13, BlockType::Granite);

        assert_eq!(chunk.block_at(0, 0, 0), BlockType::Andesite);
        assert_eq!(chunk.block_at(0, 14, 0), BlockType::PolishedAndesite);
        assert_eq!(chunk.block_at(5, 12, 13), BlockType::Granite);
    }

    #[test]
    #[should_panic]
    fn test_chunk_out_of_bounds() {
        let mut chunk = Chunk::new(ChunkPosition::new(0, 0));
        chunk.set_block_at(0, 256, 0, BlockType::Andesite);
    }
}