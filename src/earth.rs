use crate::camera::Focused;
use bevy::prelude::*;

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
                radius: RADIUS.clone(),
                sectors: 36,
                stacks: 36,
            })),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Earth)
        .insert(Focused)
        .with_children(|earth| {
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
        });
}
