use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::*;

#[derive(Component)]
pub struct Planet;

pub fn make_planets_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Planet,
        Name::new("Earth"),
        Mass { value: 5000000000000000.0 },
        RigidBody::Fixed,
        Collider::ball(50.0),
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(
            PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Circle {
                    radius: 50.0,
                    vertices: 128,
                }).unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            });
    });

    info!("Added planet");

}
