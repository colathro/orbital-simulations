use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

use camera::{pan_orbit_camera, spawn_camera};
use earth::setup_earth;
use simulation::SimulationPlugin;

mod camera;
mod earth;
mod simulation;
mod sun;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: 800.,
            height: 800.,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SimulationPlugin)
        .add_startup_system(setup_earth)
        .add_startup_system(spawn_camera)
        .add_system(pan_orbit_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;
