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

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(animate);
    }
}

fn animate(
    mut commands: Commands,
    mut anim_query: Query<(Entity, &mut Transform, &mut Animation)>
) {
    let curr_time = now();
    for (entity, mut transform, animation) in anim_query.iter_mut() {
        animation.animate(transform.as_mut(), curr_time);
        if animation.is_finished(curr_time) {
            commands.entity(entity).remove::<Animation>();
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Wrapper Struct ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //


#[derive(Component)]
pub struct Animation {
    animator: Box<dyn Animator>
}

impl Animation {

    pub fn new(animator: impl Animator) -> Self {
        Self { animator: Box::new(animator) }
    }

    pub fn animate(&self, transform: &mut Transform, curr_time: f64) {
        self.animator.update(transform, curr_time);
    }

    pub fn is_finished(&self, curr_time: f64) -> bool {
        self.animator.is_finished(curr_time)
    }

}

impl<A: Animator> From<A> for Animation {
    fn from(value: A) -> Self {
        Self::new(value)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Inner Trait ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

pub trait Animator: Send + Sync + 'static {
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

impl Animator for LinearMovement {

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



pub struct AnimationSeries {
    animators: Vec<Box<dyn Animator>>
}

impl AnimationSeries {
    pub fn new() -> Self {
        Self { animators: Vec::new() }
    }

    pub fn push(&mut self, animator: impl Animator) {
        self.animators.push(Box::new(animator))
    }
}

impl Animator for AnimationSeries {
    fn update(&self, transform: &mut Transform, time: f64) {
        for animator in &self.animators {
            // if finished go to next
            if animator.is_finished(time) {
                continue
            } else {
                // if not finished, animate and return
                animator.update(transform, time);
                return
            }

        }
        // here all finished. so animate last to get to last frame
        if let Some(animator) = self.animators.last() {
            animator.update(transform, time)
        }
    }

    fn is_finished(&self, time: f64) -> bool {
        match self.animators.last() {
            Some(animator) => animator.is_finished(time),
            None => true
        }
    }
}


// // TODO: probably rename.
pub struct HexPathingLine {
    animations: AnimationSeries
}

impl HexPathingLine {
    pub fn new(start: HexCoord, end: HexCoord, speed: f32, map: &HeightMap) -> HexPathingLine {
        let move_duration = (HEX_SMALL_DIAMETER / speed) as f64;
        let line = start.line_between(end);
        let mut animations = AnimationSeries::new();
    
        for (i, this_coord) in line.iter().enumerate() {
            let this_pos = this_coord.to_world(Some(map));
    
            if let Some(next_coord) = line.get(i + 1) {
                let next_pos = next_coord.to_world(Some(map));
                let animation = LinearMovement::new(this_pos, next_pos, speed, now() + move_duration * i as f64);
                animations.push(animation)
            }
        }
        Self { animations }
    }
}

impl Animator for HexPathingLine {
    fn update(&self, transform: &mut Transform, time: f64) {
        self.animations.update(transform, time)
    }

    fn is_finished(&self, time: f64) -> bool {
        self.animations.is_finished(time)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Utils ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ //

#[inline]
pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as f64
}