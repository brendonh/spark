use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};


#[derive(SystemDesc)]
pub struct ControlSystem;

impl<'s> System<'s> for ControlSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, input): Self::SystemData) {
        for (
            
