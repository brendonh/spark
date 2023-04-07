use std::collections::BTreeMap;

use bevy::{
    prelude::*,
    prelude::shape,
    reflect::TypeUuid,
};

use bevy_rapier2d::prelude::*;

type Pos = (i32, i32);

#[derive(Component)]
pub struct TileMarker;

#[derive(TypeUuid)]
#[uuid = "3af77720-190d-42f4-a65c-bede1fb4b01a"]
pub struct TileParts {
    mesh: Handle<Mesh>,
    materials: BTreeMap<&'static str, Handle<StandardMaterial>>
}

impl FromWorld for TileParts {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))));

        let mut materials = world.get_resource_mut::<Assets<StandardMaterial>>().unwrap();
        let mut mats = BTreeMap::new();
        mats.insert("gray", materials.add(StandardMaterial {
            base_color: Color::hex("999999").unwrap(),
            unlit: true,
            ..default()
        }));

        TileParts{
            mesh: mesh,
            materials: mats,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pos: Pos
}

impl From<Pos> for Tile {
    fn from(pos: Pos) -> Self {
        Tile{ pos }
    }
}


#[derive(Component)]
pub struct TileSet {
    pub tiles: BTreeMap<Pos, Tile>,
}


impl From<Vec<Tile>> for TileSet {
    fn from(tiles: Vec<Tile>) -> Self {
        TileSet{
            tiles: tiles.into_iter().map(|tile| {
                (tile.pos, tile)
            }).collect()
        }
    }
}

impl From<Vec<Pos>> for TileSet {
    fn from(poss: Vec<Pos>) -> Self {
        let tiles: Vec<Tile> = poss.iter().map(|&pos| Tile::from(pos)).collect();
        TileSet::from(tiles)
    }
}


pub fn make_tiles_system(
    mut commands: Commands,
    mut query: Query<(Entity, &TileSet), Added<TileSet>>,
    tile_parts: Local<TileParts>,
) {
    for (ship, tileset) in query.iter_mut() {
        for tile in tileset.tiles.values() {

            let (x, y) = tile.pos;

            let tile = commands.spawn((
                TileMarker,
                Collider::cuboid(0.5, 0.5),
                SpatialBundle {
                    transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                    ..default()
                }
            )).with_children(|parent| {
                parent.spawn(
                    PbrBundle {
                        mesh: tile_parts.mesh.clone(),
                        material: tile_parts.materials.get("gray").unwrap().clone(),
                        ..default()
                    }
                );
            }).id();

            commands.entity(ship).add_child(tile);
        }
    }
}
