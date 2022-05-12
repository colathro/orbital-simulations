use crate::camera::Focused;
use crate::simulation::{AverageRadius, Simulated};
use crate::sun::SUN_RADIUS;
use bevy::prelude::*;
use bevy::render::view::VisibleEntities;

/// Approximate radius of the earth in meters.
const RADIUS: f32 = 6371000.;

/// Approximate circumference of the earth in meters.
const CIRCUMFERENCE: f32 = 40030173.;

/// Approximate mass of the earth in kg.
const MASS: f32 = 6000000000000000000000000.0;

/// Approximate distance from the sun to earth in meters.
pub const DISTANCE_FROM_SUN: f32 = 150000000000.;

#[derive(Component)]
pub struct Earth;

pub fn rotate_earth(mut earth_query: Query<&mut Transform, With<Earth>>) {
    for mut transform in earth_query.iter_mut() {
        transform.rotation = transform.rotation * Quat::from_rotation_y(0.001);
    }
}

pub fn setup_earth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle { ..default() })
        .with_children(|sun| {
            sun.spawn_scene(asset_server.load("models/SUN.glb#Scene0"));
        })
        .insert(AverageRadius(SUN_RADIUS))
        .insert(Simulated("Sun".to_string()));

    let earth = commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(DISTANCE_FROM_SUN, 0., 0.),
                rotation: Quat::from_rotation_z(0.4101524),
                ..default()
            },
            ..default()
        })
        .insert(Earth)
        .insert(Simulated("Earth".to_string()))
        .insert(AverageRadius(RADIUS))
        .insert(Focused)
        .with_children(|earth| {
            earth.spawn_scene(asset_server.load("models/P-05W4LD.glb#Scene0"));
            // spawn debug line
            earth.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    min_x: -10000.,
                    max_x: 10000.,
                    min_y: -RADIUS * 4.,
                    max_y: RADIUS * 4.,
                    min_z: -10000.,
                    max_z: 10000.,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 1., 0.),
                    ..default()
                }),
                ..default()
            });
        })
        .id();

    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 0.82),
            illuminance: 55000.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(3. * std::f32::consts::PI / 2.),
            ..default()
        },
        visible_entities: VisibleEntities {
            entities: vec![earth],
        },
        ..default()
    });
}
