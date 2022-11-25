use bevy::prelude::*;
use rand::random;
use xxhash_rust::xxh3::xxh3_64_with_seed;

use crate::plugins::world_3d::{
    config::HEX_HEIGHT_SCALE,
    hex::HexCoord
};

// hashes bytes with seed using msg
// to distinguish it from other hashes on same bytes
pub fn seeded_hash(bytes: &[u8], seed: u64, msg: &str) -> u64 {
    let mut vec = bytes.to_vec();
    let msg_bytes =  msg.as_bytes();
    let mut msg_vec = msg_bytes.to_vec();
    vec.append(&mut msg_vec);
    xxh3_64_with_seed(vec.as_slice(), seed)
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
            generator: Box::new(RandGenerator::new(min,max,seed))
        }
    }

    pub fn new_perlin(steps: Vec<PerlinStep>, seed: Option<u64>) -> Self {
        Self {
            generator: Box::new(PerlinGenerator::new(steps, seed))
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
    seed: u64
}

impl RandGenerator {
    fn new(min: u32, max: u32, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(rand::random());
        Self {min, max, seed}
    }
}

impl HeightGenerator for RandGenerator {
    fn generate_height(&self, coord: HexCoord) -> u32 {
        let hash = seeded_hash(&coord.to_bytes(), self.seed, "Random Height Map") as u32;
        hash % (self.max - self.min) + self.min
    }
}


// generate terrain height with fractal perlin noise
struct PerlinGenerator {
    steps: Vec<PerlinStep>,
    seed: u64
}

impl PerlinGenerator {
    fn new(steps: Vec<PerlinStep>, seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or(rand::random());
        Self{steps,seed}
    }

    fn gradient(&self, vec: Vec2) -> Vec2 {
        let x_dir = seeded_hash(vec.to_string().as_bytes(), self.seed, "Perlin X Dir") as f32;
        let y_dir = seeded_hash(vec.to_string().as_bytes(), self.seed, "Perlin Y Dir") as f32;
        Vec2::new(x_dir, y_dir).normalize()
    }

    fn fade(p: f32) -> f32 {
        p*p*p*(p*(p*6. - 15.) + 10.)
    }

    fn noise(&self, v: Vec2) -> f32 {
        let v0 = v.floor();
        let v1 = v0 + Vec2::new(1.,0.);
        let v2 = v0 + Vec2::new(0.,1.);
        let v3 = v0 + Vec2::new(1.,1.);

        let g0 = self.gradient(v0);
        let g1 = self.gradient(v1);
        let g2 = self.gradient(v2);
        let g3 = self.gradient(v3);

        let t0 = v.x - v0.x;
        let t1 = v.y - v0.y;

        let fade_t0 = Self::fade(t0);
        let fade_t1 = Self::fade(t1);

        let v0v1 = (1. - fade_t0) * g0.dot(v - v0) + fade_t0 * g1.dot(v - v1);
        let v2v3 = (1. - fade_t0) * g2.dot(v - v2) + fade_t0 * g3.dot(v - v3);

        (1. - fade_t1) * v0v1 + fade_t1 * v2v3
    }
}

impl HeightGenerator for PerlinGenerator {
    fn generate_height(&self, coord: HexCoord) -> u32 {
        let mut height = 0.;
        for step in self.steps.iter() {
            let x = (coord.0 as f32) * step.x_freq;
            let y = (coord.1 as f32) * step.y_freq;
            let noise = self.noise(Vec2::new(x, y));
            height += (noise * 2. + 1.) * step.magnitude;
        }
        println!("Height Float {height}");
        println!("Height Integer {}", height as u32);
        height as u32
    }
}

pub struct PerlinStep {
    x_freq: f32,
    y_freq: f32,
    magnitude: f32
}

impl PerlinStep {
    pub fn new(x_freq: f32, y_freq: f32, magnitude: f32) -> Self {
        Self {x_freq, y_freq, magnitude}
    }
}