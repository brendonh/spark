use bevy::{
    prelude::*,
    app::AppExit,
    input::{
        ElementState,
        keyboard::KeyboardInput,
    },
};

use heron::prelude::*;

mod ships;

fn main() {

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Spark".to_string(),
            width: 1440.0,
            height: 900.0,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())

        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

        .add_startup_system(setup)
        .add_startup_system(ships::ship::make_ships_system)

        .add_system(ships::tiles::make_tiles_system)

        .add_system(exit_on_esc_system)
        .add_system(print_events)
        .run();
}

fn setup(
    mut commands: Commands,
) {

    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(50.0, 50.0, 50.0),
    //     point_light: PointLight {
    //         intensity: 600000.,
    //         range: 100.,
    //         ..default()
    //     },
    //     ..default()
    // });

    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::default(), Vec3::Y),
        orthographic_projection: OrthographicProjection {
            scale: 0.03,
            ..default()
        },
        ..OrthographicCameraBundle::new_3d()
    });
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


fn print_events(
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                println!("Entity {:?} and {:?} started to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
            }
            CollisionEvent::Stopped(data1, data2) => {
                println!("Entity {:?} and {:?} stopped to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
            }
        }
    }
}
