use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::{
        math::{Vector3},
        transform::Transform,
        Named
    },
    ecs::{Resources, World},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet},
    window::ScreenDimensions,
};

use crate::ships;

use log::info;

pub struct SpaceState;

impl SimpleState for SpaceState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;

        let sprites = load_sprite_sheet(resources);

        init_ships(world, sprites);
        init_camera(world,resources);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }
        }

        Trans::None
    }
}


fn load_sprite_sheet(resources: &mut Resources) -> Handle<SpriteSheet> {
    let loader = resources.get::<DefaultLoader>().unwrap();
    let texture = loader.load("sprites/ships.png");
    let sprites = loader.load("sprites/ships.ron");
    let sprite_sheet_store = resources.get::<ProcessingQueue<SpriteSheet>>().unwrap();
    loader.load_from_data(SpriteSheet { texture, sprites }, (), &sprite_sheet_store)
}


fn init_camera(world: &mut World, resources: &mut Resources) {
    let (width, height) = {
        let dim = resources
            .get::<ScreenDimensions>()
            .expect("Read ScreenDimensions");
        (dim.width(), dim.height())
    };

    world.push((
        Named("camera".into()),
//        Parent(player),
        Transform::from(Vector3::new(0.0, 0.0, 1.1)),
        Camera::standard_2d(width, height),
    ));
}


fn init_ships(world: &mut World, sprites: Handle<SpriteSheet>) {
    let player_sprite = SpriteRender::new(sprites.clone(), 0);

    world.push((
        Transform::default(),
        ships::Shell::new(200.0, 100.0),
        player_sprite,
        ships::ShipControl::default(),
        Named::new("player")
    ));

    let mob_sprite = SpriteRender::new(sprites.clone(), 1);
    let mut mob_transform = Transform::default();
    mob_transform.set_translation_xyz(200.0, 100.0, 0.0);

    world.push((
        mob_transform,
        ships::Shell::new(200.0, 100.0),
        mob_sprite,
    ));
}
