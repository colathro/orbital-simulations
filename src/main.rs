use bevy::{
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

use bevy_inspector_egui::InspectorPlugin;
use camera::{pan_orbit_camera, spawn_camera, switch_focus, FocusIndex, Focusable};
use earth::setup_earth;
use simulation::SimulationPlugin;
use sun::setup_sun;

mod camera;
mod earth;
mod simulation;
mod sun;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Orbital Simulations".to_string(),
            ..default()
        })
        .insert_resource(FocusIndex(0))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Focusable>::new())
        .add_plugin(SimulationPlugin)
        .add_startup_system(setup_earth)
        .add_startup_system(setup_sun)
        .add_startup_system(spawn_camera)
        .add_system(pan_orbit_camera)
        .add_system(switch_focus)
        .run();
}

#[derive(Component)]
struct MainCamera;
