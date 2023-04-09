use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::common::*;
use crate::planets::planet::Planet;
use crate::physics::orbits::*;

use crate::render::lines::*;

#[derive(Component)]
pub struct Orbital;

#[derive(Component)]
pub struct OrbitPath {
    parent: Entity
}

#[derive(Component)]
pub struct OrbitMarker {
    parent: Entity
}

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
    time: Res<Time>,
    planets: Query<(Entity, &GlobalTransform, &Mass), With<Planet>>,
    orbitals: Query<(Entity, &GlobalTransform, &Velocity), Added<Orbital>>,
) {
    let (planet, planet_transform, planet_mass) = planets.single();

    for (ship, ship_transform, ship_vel) in orbitals.iter() {
        let ship_pos = ship_transform.translation();
        let planet_pos = planet_transform.translation();

        let r = ship_pos - planet_pos;
        let v = Vec3::new(ship_vel.linvel.x, ship_vel.linvel.y, 0.0);

        info!("Relative ship pos: {:?}", ship_pos);

        let orbit = orbit_from_initial(r, v, planet_mass.value, planet, planet_pos, time.raw_elapsed());
        commands.entity(ship).insert(orbit);
    }
}


pub fn render_orbits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    orbits: Query<(Entity, &Orbit), Added<Orbit>>
) {
    for (entity, orbit) in orbits.iter() {
        let c = orbit.eccentricity * orbit.semimajor;
        let periapsis = orbit.semimajor - c;
        let apoapsis = orbit.semimajor * 2.0 - periapsis;

        info!("Orbit: {:?}, {:?}, {:?}", periapsis, apoapsis, orbit.argument.to_degrees());

        let points = orbit_to_points(orbit, 128);

        commands.spawn((
            OrbitPath{ parent: entity },
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(LineStrip {
                    points: points,
                })),
                material: materials.add(LineMaterial { color: Color::GREEN }),
                ..default()
            }));

        commands.spawn((
            OrbitMarker{ parent: entity },
            MaterialMeshBundle {
                mesh: meshes.add(shape::Circle::new(0.5).into()).into(),
                material: materials.add(LineMaterial { color: Color::RED }),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            }
        ));

    }
}


pub fn update_orbit_positions(
    time: Res<Time>,
    orbits: Query<&Orbit>,
    mut markers: Query<(&OrbitMarker, &mut Transform)>
) {
    for (marker, mut transform) in markers.iter_mut() {
        if let Ok(orbit) = orbits.get(marker.parent) {
            let time_offset = time.raw_elapsed() - orbit.initial_time;
            let (x, y, z) = calculate_position_at_time(orbit, time_offset.as_secs_f32());
            transform.translation = Vec3::new(x, z, 0.);
        } else {
            error!("No orbit! {:?}", marker.parent);
        }
    }
}
