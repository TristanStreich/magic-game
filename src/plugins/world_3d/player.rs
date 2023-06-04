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
        height_map::HeightMap,
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
    let line = HexCoord::from_world(start).line_between(end);
    let mut animations = AnimationSeries::new();

    for (i, this_coord) in line.iter().enumerate() {
        let this_pos = this_coord.to_world(Some(map));

        if let Some(next_coord) = line.get(i + 1) {
            let next_pos = next_coord.to_world(Some(map));
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    height_map: Res<HeightMap>
) {
    let material = materials.add(Color::rgb(1., 0.2, 0.2).into());

    let coord = HexCoord(0,0);
    let position = coord.to_world(Some(&height_map));
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
                mesh: asset_server.load("meshes/pieces.glb#Mesh0/Primitive0"),
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(- PLAYER_SCALE, - PLAYER_SCALE, - 10.*PLAYER_SCALE),
                    scale,
                    ..default()
                },
                ..default()
            })
            .insert(PickableBundle::default());
            parent.spawn(PbrBundle {
                mesh: asset_server.load("meshes/pieces.glb#Mesh1/Primitive0"),
                material,
                transform: Transform {
                    translation: Vec3::new(- PLAYER_SCALE, - PLAYER_SCALE, - 10.*PLAYER_SCALE),
                    scale,
                    ..default()
                },
                ..default()
            });
        });
}