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
    commands
        .spawn_bundle(PbrBundle { ..default() })
        .insert(Earth)
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
                global_transform: GlobalTransform { ..default() },
                ..default()
            });
        });
}
