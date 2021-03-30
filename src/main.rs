use amethyst::{
    assets::LoaderBundle,
    core::transform::TransformBundle,
    input::{InputBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        rendy::hal::command::ClearColor,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod states;
mod ships;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config_path = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let mut dispatcher = DispatcherBuilder::default();

    dispatcher.add_bundle(LoaderBundle);
    dispatcher.add_bundle(TransformBundle::default());
    dispatcher.add_bundle(
        InputBundle::new()
            .with_bindings_from_file(&key_bindings_path)?,
    );
    dispatcher.add_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear(ClearColor {float32: [0.0, 0.0, 0.0, 1.0],
                }),
            )
            .with_plugin(RenderFlat2D::default()),
    );

    let game = Application::new(resources, states::space::SpaceState, dispatcher)?;
    game.run();

    Ok(())
}
