use bevy::{
    prelude::*,
    app::AppExit,
    input::{
        ElementState,
        keyboard::KeyboardInput,
    },
    pbr::AmbientLight,
};

use bevy_rapier2d::{
    physics::{
        RapierConfiguration,
        RapierPhysicsPlugin,
        EventQueue,
    },
    rapier::na::Vector2,
};

mod ships;

fn main() {

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Spark".to_string(),
            width: 1440.0,
            height: 900.0,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)

        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

        .add_startup_system(setup.system())
        .add_startup_system(ships::ship::make_ships_system.system())

        .add_system(ships::tiles::make_tiles_system.system())

        .add_system(exit_on_esc_system.system())
        .add_system(print_events.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut ambient: ResMut<AmbientLight>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 0.05;
    commands.spawn_bundle(camera);

    ambient.brightness = 1.0;
    // commands.spawn_bundle(LightBundle {
    //     transform: Transform::from_xyz(5.0, 0.0, 10.0),
    //     ..Default::default()
    // });

    rapier_config.gravity = Vector2::zeros();
}



fn exit_on_esc_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ElementState::Pressed && key_code == KeyCode::Escape {
                info!("Exiting");
                app_exit_events.send(AppExit);
            }
        }
    }
}


fn print_events(events: Res<EventQueue>) {
    while let Ok(intersection_event) = events.intersection_events.pop() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}
