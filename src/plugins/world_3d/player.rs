// Bevy Imports
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::{
    PickableBundle,
    PickingEvent,
    SelectionEvent
};

use crate::plugins::world_3d::{
    animate::{
        Animation,
        LinearMovement,
        AnimationSeries,
        now,
    },
    config::{
        PLAYER_SCALE,
        PLAYER_SPEED,
        HEX_SMALL_DIAMETER,
    },
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
    mut commands: Commands,
    mut events: EventReader<PickingEvent>,
    player_query: Query<(Entity, &Transform, &Children), With<Player>>,
    tile_query: Query<&HexCoord, With<HexTile>>,
    height_map: Res<HeightMap>
) {
    let mut player_to_move: Option<Entity> = None;
    let mut move_to: Option<HexCoord> = None;
    // first check all events
    // looking for player deselected and tile selected
    for event in events.iter() {
        if let PickingEvent::Selection(selection) = event {
            match selection {
                SelectionEvent::JustSelected(e) => {
                    if let Ok(tile_coord) = tile_query.get(*e) {
                        move_to = Some(tile_coord.clone());
                    }
                },
                SelectionEvent::JustDeselected(picked_entity) => {
                    // pickable bundle is on the child entity of player so check children
                    // TODO: just select tile and then check if any player is on tile
                    for (e, _, children) in player_query.iter() {
                        if children.contains(picked_entity) {
                            player_to_move = Some(e.clone())
                        }
                    }
                }
            }
        }
    }


    // now if both player deselected and tile selected, move player to tile
    if let (Some(tile_coord), Some(player_e)) = (move_to, player_to_move) {
        let player = player_query.get(player_e);
        if let Ok((entity, transform, _)) = player {
            let animation = gen_player_movement_animation(transform.translation, tile_coord, &height_map);
            commands.entity(entity).insert(animation);
            // transform.translation = player_position(tile_coord, height);
        }
    }
}

fn gen_player_movement_animation(start: Vec3, end: HexCoord, map: &HeightMap) -> Animation {
    let move_duration = (HEX_SMALL_DIAMETER / PLAYER_SPEED) as f64;
    let line = player_pos_to_hex(start).line_between(end);
    let mut animations = AnimationSeries::new();
    for (i, this_coord) in line.iter().enumerate() {
        let this_pos = player_position(*this_coord, map.get_height(*this_coord));
        if let Some(next_coord) = line.get(i + 1) {
            let next_pos = player_position(*next_coord, map.get_height(*next_coord));
            let animation = LinearMovement::new(this_pos, next_pos, PLAYER_SPEED, now() + move_duration * i as f64);
            animations.push(animation)
        }
    }
    Animation::new(animations)
}

#[derive(Component, Inspectable)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    height_map: Res<HeightMap>
) {
    let material = materials.add(Color::rgb(1., 0.2, 0.2).into());
    let mesh_handle: Handle<Mesh> = asset_server.load("meshes/pieces.glb#Mesh0/Primitive0");

    // let mesh = assets.get_mut(&mesh_handle).unwrap();

    // // center mesh on transform
    // for attr in mesh.attributes_mut() {

    // }

    let coord = HexCoord(0,0);
    let height = height_map.get_height(coord);
    let position = player_position(coord, height);
    let scale = Vec3::splat(PLAYER_SCALE);
    commands
        .spawn(PbrBundle {
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: mesh_handle,
                material,
                transform: Transform {
                    translation: Vec3::new(- PLAYER_SCALE, - PLAYER_SCALE, - 10.*PLAYER_SCALE),
                    scale,
                    ..default()
                },
                ..default()
            })
            .insert(PickableBundle::default())
            ;
        });
}

pub fn player_position(coord: HexCoord, height: u32) -> Vec3 {
    let height = height_to_world(height);
    let mut position = coord.to_world();
    position.y = height;
    return position;
    position.y = height - PLAYER_SCALE;
    position.x -= PLAYER_SCALE;
    position.z -= 10. * PLAYER_SCALE;
    position
}

// FIXME: this function and player_position. Need to go. This is horrible.
// We need to make it so the mesh is attached to the transform properly on init
pub fn player_pos_to_hex(mut player_pos: Vec3) -> HexCoord {
    return HexCoord::from_world(player_pos);
    player_pos.z += 10. * PLAYER_SCALE;
    player_pos.x -= PLAYER_SCALE;

    HexCoord::from_world(player_pos)
}