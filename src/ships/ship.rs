use std::f32::consts::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::tiles::TileSet;
use crate::common::*;
use crate::physics::gravity::Orbital;

#[derive(Component)]
pub struct Ship;

pub fn make_ships_system(
    mut commands: Commands
) {
    let orbit_speed = (G * 2500000000000000.0 / 50.0).sqrt();
    info!("Orbit speed: {:?}", orbit_speed);
    let component_speed = FRAC_PI_4.sin() * (orbit_speed + 5.0);
    commands.spawn((
        Ship,
        Orbital,
        Name::new("Player"),
        TileSet::from(vec![(0, 1), (1, 1), (1, 0)]),
        RigidBody::Dynamic,
        Mass { value: 1.0 },
        Velocity {
            linvel: Vec2::new(component_speed, component_speed),
            ..default()
        },
        ColliderMassProperties::Density(1.0),
        Restitution{
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min
        },
        Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Min
        },
        SpatialBundle{
            transform: Transform {
                translation: Vec3::new(35.35533905932737, -35.35533905932737, 0.0),
                ..default()
            },
            ..default()
        }
    ));

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
