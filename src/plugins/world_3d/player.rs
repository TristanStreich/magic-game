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
        HexTile,
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

fn player_mover(
    mut events: EventReader<PickingEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    tile_query: Query<&HexCoord, With<HexTile>>,
    height_map: Res<HeightMap>
) {
    let mut player_to_move: Option<Entity> = None;
    let mut move_to: Option<HexCoord> = None;
    // first check all events
    // looking for player deselected and tile selected
    for event in events.iter() {
        // if let PickingEvent::Clicked(e) = event {
        //     let clicked_player = player_query.get_mut(*e);
        //     if let Ok((transform, coord)) = clicked_player {
        //         let coord = coord.into_inner();
        //         coord.0 += 1;
        //         let height = height_map.get_height(*coord);
        //         let transform = transform.into_inner();
        //         transform.translation = player_position(*coord, height);
        //     }
        // }
        if let PickingEvent::Selection(selection) = event {
            match selection {
                SelectionEvent::JustSelected(e) => {
                    if let Ok(tile_coord) = tile_query.get(*e) {
                        move_to = Some(tile_coord.clone());
                    }
                },
                SelectionEvent::JustDeselected(e) => {
                    if player_query.contains(*e) {
                        player_to_move = Some(e.clone());
                    }
                }
            }
        }
    }


    // now if both player deselected and tile selected, move player to tile
    if let Some(tile_coord) = move_to {
        if let Some(player_e) = player_to_move {
            let player = player_query.get_mut(player_e);
            if let Ok(transform) = player {
                let height = height_map.get_height(tile_coord);
                let transform = transform.into_inner();
                transform.translation = player_position(tile_coord, height);
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
    let material = materials.add(Color::rgb(1., 0.2, 0.2).into());
    let mesh: Handle<Mesh> = assets.load("meshes/pieces.glb#Mesh0/Primitive0");

    let coord = HexCoord(0,0);
    let height = height_map.get_height(coord);
    let position = player_position(coord, height);
    let scale = Vec3::splat(PLAYER_SCALE);
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform {
                translation: position,
                scale,
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player)
        .insert(PickableBundle::default());
}

pub fn player_position(coord: HexCoord, height: u32) -> Vec3 {
    let height = height_to_world(height);
    let mut position = coord.to_world();
    position.y = height - PLAYER_SCALE;
    position.x -= PLAYER_SCALE;
    position.z -= 10. * PLAYER_SCALE;
    position
}