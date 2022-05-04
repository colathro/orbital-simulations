use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};

use camera::{pan_orbit_camera, spawn_camera, PanOrbitCamera, Focused};

mod camera;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "".to_string(),
            width: 800.,
            height: 800.,
            ..default()
        })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_camera)
        .add_system(gravity)
        .add_system(pan_orbit_camera)
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // earth radius in km
    let earth_radius = 6371000.;

    wireframe_config.global = false;

    let texture_handle = asset_server.load("textures/earthmap1k.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: earth_radius.clone(),
                sectors: 100,
                stacks: 100,
                ..Default::default()
            })),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Earth)
        .insert(Focused);
}

#[derive(Component)]
pub struct Earth;

#[derive(Component)]
struct Gravity;

fn gravity() {}
