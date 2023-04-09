use std::f32::consts::*;

use bevy::{
    prelude::*,
    utils::Duration
};

use crate::common::*;

#[derive(Component)]
pub struct Orbit {
    pub planet: Entity,
    pub focus: Vec3,
    pub eccentricity: f32,
    pub semimajor: f32,
    pub argument: f32,
    pub period: f32,
    pub clockwise: bool,

    pub initial_time: Duration,
    pub initial_true_anomaly: f32
}


pub fn orbit_from_initial(r: Vec3, v: Vec3, m: f32, planet: Entity, focus: Vec3, time: Duration) -> Orbit {
    let r0 = r.length();
    let v0 = v.length();

    let mu = G * m;
    let h = r.cross(v);

    let e = (v.cross(h) / mu) - r.normalize();
    let e0 = e.length();

    let a = r0 / (2.0 - ((r0 * v0 * v0) / mu));

    let clockwise = h.z < 0.0;
    let argument = e[1].atan2(e[0]);
    let period = TAU * (a.powi(3) / mu).sqrt();

    let h2u = h.length().powi(2) / mu;

    let initial_true_anomaly = (((h2u / r0) - 1.) - e0).acos() - FRAC_PI_2;

    return Orbit {
        planet: planet,
        focus: focus,
        eccentricity: e0,
        semimajor: a,
        argument: argument,
        period: period,
        clockwise: clockwise,

        initial_time: time,
        initial_true_anomaly: initial_true_anomaly
    };
}

pub fn orbit_to_points(orbit: &Orbit, points: u32) -> Vec<Vec3> {
    let phi = orbit.argument;
    let step = TAU / (points - 1) as f32;
    let semi_rectum = orbit.semimajor * (1.0 - orbit.eccentricity.powi(2));
    return (0..points).map(|i| {
        let theta = i as f32 * step;
        let radius = semi_rectum / (1.0 - (orbit.eccentricity * (theta - phi).cos()));
        return Vec3::new(radius * theta.sin(), radius * theta.cos(), 0.0);
    }).collect::<Vec<Vec3>>();
}

// https://github.com/atbentley/bevy_mod_orbits/blob/main/src/math.rs

#[inline]
pub fn calculate_position_at_time(
    orbit: &Orbit,
    time: f32,
) -> (f32, f32, f32) {
    let mean_motion = calculate_mean_motion(orbit.period);
    let mean_anomaly = calculate_mean_anomaly(mean_motion, orbit.initial_true_anomaly, time);
    let eccentric_anomaly = calculate_eccentric_anomaly(orbit.eccentricity, mean_anomaly);
    let mut true_anomaly = calculate_true_anomaly(orbit.eccentricity, eccentric_anomaly);
    if !orbit.clockwise { true_anomaly *= -1. };
    let heliocentric_distance = calculate_heliocentric_distance(orbit.semimajor, orbit.eccentricity, true_anomaly);
    calculate_position(true_anomaly, heliocentric_distance, orbit.argument, mean_anomaly)
}

#[inline]
pub fn calculate_mean_motion(period: f32) -> f32 {
    TAU / period
}

#[inline]
pub fn calculate_mean_anomaly(mean_motion: f32, initial_mean_anomaly: f32, time: f32) -> f32 {
    (initial_mean_anomaly + mean_motion * time).rem_euclid(TAU)
}

#[inline]
pub fn calculate_initial_mean_anomaly(mean_anomaly: f32, period: f32, time: f32) -> f32 {
    let mean_motion = calculate_mean_motion(period);
    (mean_anomaly - mean_motion * time).rem_euclid(TAU)
}

#[inline]
pub fn calculate_eccentric_anomaly(eccentricity: f32, mean_anomaly: f32) -> f32 {
    let e = eccentricity;
    let ma = mean_anomaly;
    let mut ea = ma;
    // using Newton's method
    for _i in 0..5 {
        ea = ea - (ea - e * ea.sin() - ma) / (1.0 - e * ea.cos());
    }
    ea
}

#[inline]
pub fn calculate_true_anomaly(eccentricity: f32, eccentric_anomaly: f32) -> f32 {
    let e = eccentricity;
    let e_a = eccentric_anomaly;
    2.0 * (((1.0 + e) / (1.0 - e) * ((e_a / 2.0).tan()).powi(2)).sqrt()).atan()
}

#[inline]
pub fn calculate_heliocentric_distance(semi_major_axis: f32, eccentricity: f32, true_anomaly: f32) -> f32 {
    let semilatus_rectum = semi_major_axis * (1.0 - eccentricity.powi(2));
    semilatus_rectum / (1.0 + eccentricity * true_anomaly.cos())
}

#[inline]
pub fn calculate_position(
    true_anomaly: f32,
    heliocentric_distance: f32,
    argument_of_periapsis: f32,
    mean_anomaly: f32,
) -> (f32, f32, f32) {
    let zmod = if (mean_anomaly % TAU) < PI { -1.0 } else { 1.0 };

    let x = heliocentric_distance * true_anomaly.cos();
    let z = heliocentric_distance * true_anomaly.sin() * zmod;

    let rotated_x = x * argument_of_periapsis.cos() - z * argument_of_periapsis.sin();
    let rotated_z = x * argument_of_periapsis.sin() + z * argument_of_periapsis.cos();

    (rotated_x, 0.0, rotated_z)
}
