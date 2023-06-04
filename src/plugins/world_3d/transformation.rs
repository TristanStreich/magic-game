//! Plugin for handling entity movement
use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::plugins::world_3d::{
    hex::{
        HexCoord,
        height_map::HeightMap,
    },
    config::HEX_SMALL_DIAMETER,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ System ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

pub struct TransformationPlugin;
impl Plugin for TransformationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(transformation_driver);
    }
}

fn transformation_driver(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Transformation)>
) {
    let curr_time = now();
    for (entity, mut transform, transformation) in query.iter_mut() {
        transformation.update(&mut transform, curr_time);
        if transformation.is_finished(curr_time) {
            commands.entity(entity).remove::<Transformation>();
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Wrapper Struct ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[derive(Component)]
/// Wrapper Struct for Transformer which allows Transformers to be queried as a component
pub struct Transformation {
    transformer: Box<dyn Transformer>
}

impl Transformation {

    pub fn new(transformer: impl Transformer) -> Self {
        Self { transformer: Box::new(transformer) }
    }

    pub fn update(&self, transform: &mut Transform, curr_time: f64) {
        self.transformer.update(transform, curr_time);
    }

    pub fn is_finished(&self, curr_time: f64) -> bool {
        self.transformer.is_finished(curr_time)
    }

}

impl<T: Transformer> From<T> for Transformation {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Inner Trait ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

pub trait Transformer: Send + Sync + 'static {
    /// Edits a transform based on a time.
    /// 
    /// time: unix_time in ms
    /// 
    /// If a time that is passed in is after the ending time of this transformer then
    /// the transformer should update the transformer to min(time, transformer.end_time)
    /// rather than going past its desired ending position
    fn update(&self, transform: &mut Transform, time: f64);
    fn is_finished(&self, time: f64) -> bool;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~ Trait Implementors ~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

#[derive(Debug)]
pub struct LinearMovement {
    start_time: f64,
    end_time: f64,
    start_pos: Vec3,
    velocity: Vec3
}

impl LinearMovement {
    pub fn new(start_pos: Vec3, end_pos: Vec3, speed: f32, start_time: f64) -> Self {
        let path = end_pos - start_pos;
        let dir = path.normalize();
        let velocity = dir * speed;
        let duration = (path.length() / speed) as f64;
        let end_time = duration + start_time;
        LinearMovement {
            start_time,
            end_time,
            start_pos,
            velocity
        }
    }
}

impl Transformer for LinearMovement {

    fn update(&self, transform: &mut Transform, time: f64) {
        let time = f64::min(time, self.end_time);
        let dur = time - self.start_time;
        let curr_pos = self.start_pos + self.velocity * dur as f32;
        transform.translation = curr_pos;
    }

    fn is_finished(&self, time: f64) -> bool {
        time >= self.end_time
    }
}


pub struct TransformerSeries {
    transformers: Vec<Box<dyn Transformer>>
}

impl TransformerSeries {
    pub fn new() -> Self {
        Self { transformers: Vec::new() }
    }

    pub fn push(&mut self, transformer: impl Transformer) {
        self.transformers.push(Box::new(transformer))
    }
}

impl Transformer for TransformerSeries {
    fn update(&self, transform: &mut Transform, time: f64) {
        for transformer in &self.transformers {
            // if finished go to next
            if transformer.is_finished(time) {
                continue
            } else {
                // if not finished, update and return
                transformer.update(transform, time);
                return
            }

        }
        // here all finished. so update last to get to final state
        if let Some(transformer) = self.transformers.last() {
            transformer.update(transform, time)
        }
    }

    fn is_finished(&self, time: f64) -> bool {
        match self.transformers.last() {
            Some(transformer) => transformer.is_finished(time),
            None => true
        }
    }
}

/// Moves piece from its starting coord to another coord,
/// moving to intermediate tiles along a straight line bewteen the two
pub struct HexPathingLine {
    transformers: TransformerSeries
}

impl HexPathingLine {
    pub fn new(start: HexCoord, end: HexCoord, speed: f32, map: &HeightMap) -> HexPathingLine {
        let move_duration = (HEX_SMALL_DIAMETER / speed) as f64;
        let line = start.line_between(end);
        let mut transformers = TransformerSeries::new();
    
        for (i, this_coord) in line.iter().enumerate() {
            let this_pos = this_coord.to_world(Some(map));
    
            if let Some(next_coord) = line.get(i + 1) {
                let next_pos = next_coord.to_world(Some(map));
                let transformer = LinearMovement::new(this_pos, next_pos, speed, now() + move_duration * i as f64);
                transformers.push(transformer)
            }
        }
        Self { transformers }
    }
}

impl Transformer for HexPathingLine {
    fn update(&self, transform: &mut Transform, time: f64) {
        self.transformers.update(transform, time)
    }

    fn is_finished(&self, time: f64) -> bool {
        self.transformers.is_finished(time)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Utils ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

#[inline]
pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64
}