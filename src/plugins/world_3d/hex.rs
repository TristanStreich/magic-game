// Standard Lib Imports
use std::cmp::{max,min};

// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const HEX_INNER_RADIUS: f32 = 1.0; // 40
pub const HEX_CIRCUMRADIUS: f32 = HEX_INNER_RADIUS * 1.154700538; //sqrt(4/3)
pub const HEX_SMALL_DIAMETER: f32 = 2.0 * HEX_INNER_RADIUS;
pub const HEX_LARGE_DIAMETER: f32 = 2.0 * HEX_CIRCUMRADIUS;
pub const HEX_GRID_RADIUS: i32 = 5;
pub const HEX_SPRITE_SCALE: f32 = HEX_SMALL_DIAMETER * 0.00275;

pub struct HexPlugin;

impl Plugin for HexPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_3d_hex_grid);
    }
}

#[derive(Component, Inspectable, Debug)]
pub struct HexCoord(pub i32, pub i32);

impl HexCoord {
    pub fn for_carter(&self) -> Vec3 {
        let x = HEX_CIRCUMRADIUS * f32::sqrt(3.0) * ((self.0 as f32) + (self.1 as f32) / 2.0);
        let y = HEX_CIRCUMRADIUS * (3.0/2.0) * (self.1 as f32);
        return Vec3 { x: x, y: 0.0, z: y };
    }

    pub fn clone(&self) -> Self {
        HexCoord(self.0,self.1)
    }

    // returns all the hex coords that are
    // within radius number of tiles
    pub fn within_radius(&self, radius: i32) -> Vec<HexCoord> {
        let mut within = Vec::new();
        for x in -radius..radius+1 {
            for y in max(-radius, (-x)-radius)..min(radius,(-x)+radius)+1 {
                within.push(HexCoord(x+self.0, y+self.1));
            }
        }
        return within;
    }
}

impl PartialEq for HexCoord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

pub fn setup_3d_hex_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let hex_3d: Handle<Mesh> = asset_server.load("hex.glb#Mesh0/Primitive0");

    for hex_coord in HexCoord(0,0).within_radius(HEX_GRID_RADIUS).into_iter() {
        let position = hex_coord.for_carter();

        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            // Add children to the parent
            .with_children(|parent| {
                parent.spawn_bundle(PbrBundle {
                    mesh: hex_3d.clone(),
                    material: white_material.clone(),
                    // transform: {
                    //     let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                    //     transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    //     transform
                    // },
                    ..Default::default()
                });
            });
    }
}
