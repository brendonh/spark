use std::collections::BTreeMap;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
};

use bevy_rapier2d::{
    rapier::geometry::{SharedShape, ColliderBuilder},
    na::Isometry2,
};

type Pos = (i32, i32);

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
        mats.insert("gray", materials.add(Color::rgb(0.6, 0.6, 0.6).into()));

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


#[derive(Clone)]
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


impl From<&TileSet> for SharedShape {
    fn from(tileset: &TileSet) -> Self {
        let shapes = tileset.tiles.values().map(|tile| {
            let (x, y) = tile.pos;
            (Isometry2::translation(x as f32, y as f32), SharedShape::cuboid(0.5, 0.5))
        }).collect();
        SharedShape::compound(shapes)
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
            let collider = ColliderBuilder::new(tileset.into());

            commands.spawn()
                .insert(TileMarker)
                .insert(Parent(ship))
                .insert(collider)
                .insert_bundle(
                    PbrBundle {
                        mesh: tile_parts.mesh.clone(),
                        material: tile_parts.materials.get("gray").unwrap().clone(),
                        transform: Transform::from_xyz(x as f32, y as f32, 0.0),
                        ..Default::default()
                    });
        }
    }
}
