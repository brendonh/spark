use bevy::prelude::*;
use heron::prelude::*;

use super::tiles::TileSet;
use crate::common::Orbital;

#[derive(Component)]
pub struct Ship;

pub fn make_ships_system(
    mut commands: Commands,
) {
    commands.spawn_bundle((
        Ship,
        Orbital,
        Name::new("Player"),
        TileSet::from(vec![(0, 1), (1, 1), (1, 0)]),
        RigidBody::Dynamic,
        Velocity {
            linear: Vec3::X * -4.3,
            ..default()
        },
        PhysicMaterial {
            restitution: 1.0,
            density: 1.0,
            friction: 0.0,
        }
    )).insert_bundle(
        TransformBundle {
            local: Transform {
                translation: Vec3::new(0.0, 10.0, 0.0),
                ..default()
            },
            ..default()
        }
    );

    // commands.spawn_bundle((
    //     Ship,
    //     Name::new("Other"),
    //     TileSet::from(vec![(0,0), (0, 1), (1, -1)]),
    //     RigidBody::Dynamic,
    //     Velocity {
    //         angular: AxisAngle::new(Vec3::Y, 3.0),
    //         ..default()
    //     }
    // )).insert_bundle(
    //     TransformBundle {
    //         local: Transform {
    //             translation: Vec3::new(1.0, 2.5, 0.0),
    //             ..default()
    //         },
    //         ..default()
    //     });

}
