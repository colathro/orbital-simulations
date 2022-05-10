use crate::camera::Focused;
use crate::simulation::AverageRadius;
use crate::sun::{DISTANCE_FROM_SUN, SUN_RADIUS};
use bevy::prelude::*;
use bevy::render::view::VisibleEntities;

const RADIUS: f32 = 6371000.;
const CIRCUMFERENCE: f32 = 40030173.;

#[derive(Component)]
pub struct Earth;

pub fn setup_earth(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: SUN_RADIUS,
            ..default()
        })),
        material: materials.add(StandardMaterial {
            base_color: Color::YELLOW,
            ..default()
        }),
        ..default()
    });

    let earth = commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(DISTANCE_FROM_SUN, 0., 0.),
            ..default()
        })
        .insert(Earth)
        .insert(Focused)
        .insert(AverageRadius(RADIUS))
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
