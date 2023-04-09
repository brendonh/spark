use bevy::{
    prelude::*,
    app::{AppExit},
    input::{
        ButtonState,
        keyboard::KeyboardInput,
    },
    time::common_conditions::on_timer,
    utils::Duration,
//    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

use bevy_rapier2d::prelude::*;

mod common;
mod ships;
mod planets;
mod physics;
mod render;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Spark".to_string(),
                resolution: bevy::window::WindowResolution::new(1440.0, 900.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(MaterialPlugin::<render::lines::LineMaterial>::default())

        .insert_resource(RapierConfiguration{
            gravity: Vec2::ZERO,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

        .add_startup_system(setup)
        .add_startup_system(planets::planet::make_planets_system)
        .add_startup_system(ships::ship::make_ships_system)

        .add_system(ships::tiles::make_tiles_system)
        .add_system(physics::gravity::add_gravity)
        .add_system(physics::gravity::calc_orbits)
        .add_system(physics::gravity::render_orbits)
        .add_system(physics::gravity::update_orbit_positions)

        .add_system(physics::gravity::apply_gravity.before(bevy_rapier2d::plugin::PhysicsSet::StepSimulation))

        .add_system(exit_on_esc_system)

        .add_system(physics::gravity::log_distances.run_if(on_timer(Duration::from_secs_f32(0.5))))

        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())

//        .add_system(print_events)
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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: OrthographicProjection {
            scale: 200.0,
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(1.0),
            ..default()
        }.into(),
        ..default()
    });

}

fn exit_on_esc_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for event in keyboard_input_events.iter() {
        if let Some(key_code) = event.key_code {
            if event.state == ButtonState::Pressed && key_code == KeyCode::Escape {
                info!("Exiting");
                app_exit_events.send(AppExit);
            }
        }
    }
}


// fn print_events(
//     mut events: EventReader<CollisionEvent>,
// ) {
//     for event in events.iter() {
//         match event {
//             CollisionEvent::Started(entity1, entity2, flags) => {
//                 println!("Entity {:?} and {:?} started to collide: {:?}", entity1, entity2, flags)
//             }
//             CollisionEvent::Stopped(entity1, entity2, flags) => {
//                 println!("Entity {:?} and {:?} stopped colliding: {:?}", entity1, entity2, flags)
//             }
//         }
//     }
// }
