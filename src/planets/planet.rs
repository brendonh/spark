use bevy::prelude::*;
use heron::prelude::*;

use crate::common::*;

#[derive(Component)]
pub struct Planet;

pub fn make_planets_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle((
        Planet,
        Name::new("Earth"),
        Mass { value: 5.0 },
        RigidBody::Static,
        CollisionShape::Sphere {
            radius: 3.0,
        }
    )).insert_bundle(
        TransformBundle {
            local: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn_bundle(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 3.0,
                    subdivisions: 32,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            }
        );
    });

}


pub fn apply_gravity(
    planets: Query<(&Mass, &GlobalTransform), With<Planet>>,
    mut orbitals: Query<(&mut Velocity, &GlobalTransform), With<Orbital>>,
) {
    for (mass, planet_pos) in planets.iter() {
        for (mut velocity, ship_pos) in orbitals.iter_mut() {
            let vector = planet_pos.translation - ship_pos.translation;
            let gravity = mass.value / vector.length_squared();

            // This is silly
            velocity.linear += vector.normalize() * gravity;

        }
    }
}
