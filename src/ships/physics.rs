use amethyst::{
    core::math::{Rotation2},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Physics {
    pub spin: Rotation2<f32>,
}

impl Physics {
    pub fn new() -> Physics {
        Physics {
            spin: Rotation2::one()
        }
    }
}
