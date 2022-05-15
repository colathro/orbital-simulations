use crate::simulation::{HPVec3, PhysicalProperties, ReferenceFrame, Simulated};
use bevy::prelude::*;
use rug::Float;

/// Approximate radius of the earth in meters.
const RADIUS: f32 = 6.96e+8_f32;

/// Approximate mass of the earth in kg.
const MASS: f32 = 1.989e+30_f32;

#[derive(Component)]
pub struct Sun;

pub fn setup_sun(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(PbrBundle { ..default() })
        .with_children(|sun| {
            sun.spawn_scene(asset_server.load("models/SUN.glb#Scene0"));
        })
        .insert(PhysicalProperties {
            mass: Float::with_val(128, MASS),
            estimated_radius: Float::with_val(128, RADIUS),
            acceleration: HPVec3::zero(),
            translation: HPVec3::zero(),
        })
        .insert(Simulated("Sun".to_string()))
        .insert(ReferenceFrame);

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
        ..default()
    });
}
