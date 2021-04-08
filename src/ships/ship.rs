use bevy::{
    prelude::*,
};

use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
};

use super::tiles::TileSet;

pub struct Ship;

pub fn make_ships_system(
    mut commands: Commands,
) {
    commands.spawn()
        .insert_bundle((
            Ship,
            Name::new("Player"),
            TileSet::from(vec![(0, 1), (1, 1), (1, 0)]),
            Transform::identity(),
            GlobalTransform::identity(),
            RigidBodyBuilder::new_dynamic()
                .translation(0.0, 0.0)
                .linvel(5.0, 0.0),
        ));

    commands.spawn()
        .insert_bundle((
            Ship,
            Name::new("Other"),
            TileSet::from(vec![(0,0), (0, 1), (1, -1)]),
            Transform::identity(),
            GlobalTransform::identity(),
            RigidBodyBuilder::new_dynamic()
                .translation(10.0, 2.5)
                .linvel(-5.0, 0.0),
        ));

}
