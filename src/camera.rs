use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

use crate::{earth::DISTANCE_FROM_SUN, simulation::PhysicalProperties};

/// Sample code taken from: https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html
/// Edited for my needs :D

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

#[derive(Component)]
pub struct Focused;

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    input_keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
    focused_query: Query<(&GlobalTransform, &PhysicalProperties), With<Focused>>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Left;

    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut orbit_cam, mut transform) in query.iter_mut() {
        match focused_query.get_single() {
            Ok((focused_transform, physical_properties)) => {
                orbit_cam.focus = focused_transform.translation;
                let _ = transform.looking_at(focused_transform.translation, Vec3::Y);
                if input_keyboard.pressed(KeyCode::Space) {
                    transform.translation = focused_transform.translation;
                    orbit_cam.radius = physical_properties.estimated_radius.to_f32() * 4.;
                    scroll += 0.000001;
                }
            }
            Err(_) => {}
        }

        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            orbit_cam.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if orbit_cam.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if scroll.abs() > 0.0 {
            any = true;
            orbit_cam.radius -= scroll * orbit_cam.radius * 0.01;
            // dont allow zoom to reach zero or you get stuck
            orbit_cam.radius = f32::max(orbit_cam.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                orbit_cam.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit_cam.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Spawn a camera like this
pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            camera: Camera {
                far: DISTANCE_FROM_SUN * 2.,
                ..default()
            },
            ..default()
        })
        .insert(PanOrbitCamera {
            ..Default::default()
        });

    commands.spawn_bundle(UiCameraBundle::default());
}
