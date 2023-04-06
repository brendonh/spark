use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
        Mass { value: 5.0 },
        RigidBody::Fixed,
        Collider::ball(3.0),
        SpatialBundle::default(),
    )).with_children(|parent| {
        parent.spawn(
            PbrBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                mesh: meshes.add(Mesh::try_from(shape::Icosphere {
                    radius: 3.0,
                    subdivisions: 32,
                }).unwrap()),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("ffd891").unwrap(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            });
    });

}


pub fn apply_gravity(
    planets: Query<(&Mass, &GlobalTransform), With<Planet>>,
    mut orbitals: Query<(&mut Velocity, &GlobalTransform), With<Orbital>>,
) {
    for (mass, planet_pos) in planets.iter() {
        for (mut velocity, ship_pos) in orbitals.iter_mut() {
            let vector = planet_pos.translation() - ship_pos.translation();
            let gravity = mass.value / vector.length_squared();

            // This is silly
            velocity.linvel += vector.normalize() * gravity;

        }
    }
}
