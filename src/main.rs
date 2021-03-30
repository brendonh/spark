use bevy::{
    prelude::*,
    asset::LoadState,
    sprite::TextureAtlasBuilder,
};

// mod states;
// mod ships;

fn main() {

    static STATE: &str = "what";

    App::build()
        .add_resource(WindowDescriptor {
            title: "Spark".to_string(),
            ..Default::default()
        })

        .init_resource::<Sprites>()

        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Setup))

        .add_stage_before(
            stage::UPDATE, STATE,
            StateStage::<AppState>::default()
        )

        .on_state_enter(STATE, AppState::Setup, load_textures.system())
        .on_state_update(STATE, AppState::Setup, check_textures.system())

        .on_state_enter(STATE, AppState::Space, setup.system())

        .run();
}

#[derive(Debug, Clone)]
enum AppState {
    Setup,
    Space,
}

#[derive(Default)]
struct Sprites {
    handles: Vec<HandleUntyped>
}

fn load_textures(mut sprites: ResMut<Sprites>, asset_server: Res<AssetServer>) {
    info!("Loading ship sprites");
    sprites.handles = asset_server.load_folder("sprites/ships").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    sprites: ResMut<Sprites>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprites.handles.iter().map(|handle| handle.id))
    {
        info!("Load finished");
        state.set_next(AppState::Space).unwrap();
    } else {
        warn!("Still loading ...");
    }
}


fn setup(
    commands: &mut Commands,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in sprites.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

    let player_handle = asset_server.get_handle("sprites/ships/1B.png");
    let player_index = texture_atlas.get_texture_index(&player_handle).unwrap();

    warn!("Player sprite: {:?}", player_index);

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(player_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        });
}

// amethyst::start_logger(Default::default());

//     let app_root = application_root_dir()?;

//     let resources = app_root.join("assets");
//     let display_config_path = app_root.join("config/display_config.ron");
//     let key_bindings_path = app_root.join("config/input.ron");

//     let mut dispatcher = DispatcherBuilder::default();

//     dispatcher.add_bundle(LoaderBundle);
//     dispatcher.add_bundle(TransformBundle::default());
//     dispatcher.add_bundle(
//         InputBundle::new()
//             .with_bindings_from_file(&key_bindings_path)?,
//     );
//     dispatcher.add_bundle(
//         RenderingBundle::<DefaultBackend>::new()
//             .with_plugin(
//                 RenderToWindow::from_config_path(display_config_path)?
//                     .with_clear(ClearColor {float32: [0.0, 0.0, 0.0, 1.0],
//                 }),
//             )
//             .with_plugin(RenderFlat2D::default()),
//     );

//     let game = Application::new(resources, states::space::SpaceState, dispatcher)?;
//     game.run();

//     Ok(())
// }
