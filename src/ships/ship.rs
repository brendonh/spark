use bevy::{
    prelude::*,
};


use heron::prelude::*;

use super::tiles::TileSet;

#[derive(Component)]
pub struct Ship;

pub fn make_ships_system(
    mut commands: Commands,
) {
    commands.spawn_bundle((
        Ship,
        Name::new("Player"),
        TileSet::from(vec![(0, 1), (1, 1), (1, 0)]),
        RigidBody::Dynamic,
        Velocity::from_linear(Vec3::Y * -5.0)
    )).insert_bundle(
        TransformBundle {
            local: Transform {
                translation: Vec3::new(1.5, 10.0, 0.0),
                ..default()
            },
            ..default()
        }
    );

    commands.spawn_bundle((
        Ship,
        Name::new("Other"),
        TileSet::from(vec![(0,0), (0, 1), (1, -1)]),
        RigidBody::Dynamic,
    )).insert_bundle(
        TransformBundle {
            local: Transform {
                translation: Vec3::new(1.0, 2.5, 0.0),
                ..default()
            },
            ..default()
        });

}
