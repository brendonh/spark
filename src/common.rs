use bevy::prelude::*;

#[derive(Component)]
pub struct Orbital;

#[derive(Component)]
pub struct Mass {
    pub value: f32
}

pub const G: f32 = 6.67430e-11;
