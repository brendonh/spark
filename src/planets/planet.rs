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
        Mass { value: 5000000000000.0 },
        RigidBody::Fixed,
        Collider::ball(3.0),
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
                //transform: Transform::from_xyz(0.0, 0.0, 0.0),
                mesh: meshes.add(Mesh::try_from(shape::Circle {
                    radius: 3.0,
                    vertices: 32,
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
    fixed_time: Res<FixedTime>
) {
    let delta = fixed_time.period.as_secs_f32();
    for (planet_mass, planet_pos) in planets.iter() {
        for (mut velocity, ship_pos) in orbitals.iter_mut() {
            let vector = planet_pos.translation() - ship_pos.translation();
            let gravity = G * planet_mass.value / vector.length_squared();
            let direction = vector.normalize();

            velocity.linvel.x += direction.x * gravity * delta;
            velocity.linvel.y += direction.y * gravity * delta;
        }
    }
}



pub fn log_distances(
    planets: Query<(&Mass, &GlobalTransform), With<Planet>>,
    mut orbitals: Query<(&mut Velocity, &GlobalTransform), With<Orbital>>
) {
    for (planet_mass, planet_pos) in planets.iter() {
        for (mut velocity, ship_pos) in orbitals.iter_mut() {
            let vector = planet_pos.translation() - ship_pos.translation();
            info!("Dist: {:?}", vector.length());
        }
    }
}
