use std::f32::consts::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::*;
use crate::planets::planet::Planet;
use crate::physics::orbits::*;

use crate::render::lines::*;

#[derive(Component)]
pub struct Orbital;

pub fn add_gravity(
    mut commands: Commands,
    mut query: Query<Entity, Added<Orbital>>
) {
    for entity in query.iter_mut() {
        commands.entity(entity)
            .insert(ExternalForce{
                force: Vec2::new(0.0, -1.5),
                torque: 0.0
            })
            .insert(ReadMassProperties::default());
    }
}


pub fn apply_gravity(
    planets: Query<(&GlobalTransform, &Mass), With<Planet>>,
    mut forces: Query<(&GlobalTransform, &ReadMassProperties, &mut ExternalForce), With<Orbital>>
) {
    for (planet_pos, planet_mass) in planets.iter() {
        for (ship_pos, mass_props, mut ext_force) in forces.iter_mut() {
            let vector = planet_pos.translation() - ship_pos.translation();
            let gravity = (G * planet_mass.value / vector.length_squared()) * vector.normalize();
            let force = gravity * mass_props.0.mass;

            ext_force.force = Vec2::new(force.x, force.y);
        }
    }
}



pub fn log_distances(
    planets: Query<&GlobalTransform, With<Planet>>,
    orbitals: Query<&GlobalTransform, With<Orbital>>
) {
    for planet_pos in planets.iter() {
        for ship_pos in orbitals.iter() {
            let vector = planet_pos.translation() - ship_pos.translation();
            info!("Dist: {:?}", vector.length());
        }
    }
}


pub fn calc_orbits(
    mut commands: Commands,
    planets: Query<(Entity, &GlobalTransform, &Mass), With<Planet>>,
    orbitals: Query<(Entity, &GlobalTransform, &Velocity), Added<Orbital>>
) {
    let (planet, planet_pos, planet_mass) = planets.single();

    for (ship, ship_pos, ship_vel) in orbitals.iter() {
        let r = ship_pos.translation() - planet_pos.translation();
        let v = Vec3::new(ship_vel.linvel.x, ship_vel.linvel.y, 0.0);

        info!("Relative ship pos: {:?}", ship_pos.translation());


        let mut orbit = orbit_from_initial(r, v, planet_mass.value);
        orbit.planet = planet;
        commands.entity(ship).insert(orbit);
    }
}


pub fn render_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    orbits: Query<&Orbit, Changed<Orbit>>
) {
    for orbit in orbits.iter() {
        let c = orbit.eccentricity * orbit.semimajor;
        let semiminor = orbit.semimajor * (1.0 - orbit.eccentricity.powi(2)).sqrt();

        let periapsis = orbit.semimajor - c;
        let apoapsis = orbit.semimajor * 2.0 - periapsis;

        info!("Orbit: {:?}, {:?}, {:?}", periapsis, apoapsis, orbit.argument);

        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(LineStrip {
                points: vec![
                    Vec3::new(0.0, -periapsis, 0.0),
                    Vec3::new(0.0, apoapsis, 0.0),
                ],
            })),
            transform: Transform {
                rotation: Quat::from_rotation_z(orbit.argument + FRAC_PI_2),
                ..default()
            },
            material: materials.add(LineMaterial { color: Color::GREEN }),
            ..default()
        });

    }
}
