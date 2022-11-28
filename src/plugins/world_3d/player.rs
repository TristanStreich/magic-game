// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{
    PickableBundle,
    PickingEvent,
    SelectionEvent
};

use crate::plugins::world_3d::{
    config::PLAYER_SCALE,
    hex::{
        HexCoord,
        height_map::{
            to_world as height_to_world,
            HeightMap
        }
    }
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(spawn_player)
        .add_system(player_mover)
        ;
    }
}

fn player_mover(mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        if let PickingEvent::Selection(selection) = event {
            match selection {
                SelectionEvent::JustSelected(e) => println!("Just Selected: {e:?}"),
                SelectionEvent::JustDeselected(e) => println!("Just Deselected: {e:?}")
            }
        }
    }
}

#[derive(Component, Inspectable)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    height_map: Res<HeightMap>
) {
    let material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let mesh: Handle<Mesh> = assets.load("meshes/pieces.glb#Mesh0/Primitive0");

    let coord = HexCoord(0,0);
    let height = height_map.get_height(coord);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: player_transform(coord, height),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(PickableBundle::default());
}

pub fn player_transform(coord: HexCoord, height: u32) -> Transform {
    let height = height_to_world(height);
    let mut position = coord.to_world();
    let scale = Vec3::splat(PLAYER_SCALE);
    position.y = height - PLAYER_SCALE;
    position.x -= PLAYER_SCALE;
    position.z -= 10. * PLAYER_SCALE;
    Transform {
        translation: position,
        scale,
        ..default()
    }
}