use bevy::prelude::*;
use rand::random;
use xxhash_rust::xxh3::xxh3_64_with_seed;

use crate::plugins::world_3d::{
    config::HEX_HEIGHT_SCALE,
    hex::HexCoord
};

pub struct Hasher{
    seed: u64
}
impl Hasher {
    pub fn new(seed: Option<u64>) -> Self {
        let seed = match seed {
            Some(seed) => seed,
            None => random()
        };
        Self {seed}
    }

    pub fn hash(&self, coord: HexCoord) -> u32 {
        xxh3_64_with_seed(&coord.to_bytes(), self.seed) as u32
    }

    pub fn hash_range(&self, coord: HexCoord, min: u32, max: u32) -> u32 {
        let mut hash = self.hash(coord);
        hash %= max - min;
        hash + min
    }
}

pub fn to_world(height: u32) -> f32 {
    (height as f32) * HEX_HEIGHT_SCALE
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Wrapper Struct ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

#[derive(Resource)]
pub struct HeightMap {
    generator: Box<dyn HeightGenerator>
}

impl HeightMap {
    pub fn get_height(&self, coord: HexCoord) -> u32 {
        self.generator.generate_height(coord)
    }

    pub fn new_rand(min: u32, max: u32, seed: Option<u64>) -> Self {
        Self {
            generator: Box::new(RandGenerator {
                min,
                max,
                hasher: Hasher::new(seed)
            })
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Inner Trait  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

trait HeightGenerator: Send + Sync + 'static {
    fn generate_height(&self, coord: HexCoord) -> u32;
}


struct RandGenerator {
    min: u32,
    max: u32,
    hasher: Hasher
}

impl HeightGenerator for RandGenerator {
    fn generate_height(&self, coord: HexCoord) -> u32 {
        self.hasher.hash_range(coord, self.min, self.max)
    }
}


struct HillsGenerator {

}

