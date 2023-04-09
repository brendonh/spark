use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::common::*;

#[derive(Component)]
pub struct Orbit {
    pub planet: Entity,
    pub eccentricity: f32,
    pub semimajor: f32,
    pub argument: f32
}


pub fn orbit_from_initial(r: Vec3, v: Vec3, m: f32) -> Orbit {
    let r0 = r.length();
    let v0 = v.length();

    let mu = G * m;
    let h = r.cross(v);

    let e = (v.cross(h) / mu) - r.normalize();
    let a = r0 / (2.0 - ((r0 * v0 * v0) / mu));

    // Only works in 2D!
    let mut argument = e[1].atan2(e[0]);
    if h.z < 0.0 { argument = TAU - argument };

    info!("Calc: e: {:?}, a: {:?}", e.length(), a);
    return Orbit {
        planet: Entity::PLACEHOLDER,
        eccentricity: e.length(),
        semimajor: a,
        argument: argument
    };


    // info!("Eccentricity: {:?}, {:?}", e0, e);
    // info!("Periapsis: {:?}", periapsis);
    // info!("Apoapsis: {:?}", apoapsis);
    // info!("Argument: {:?}", argument.to_degrees());

}
