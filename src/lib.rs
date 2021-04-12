//! An orbit controls plugin for bevy.
//!
//! # Usage
//!
//! Register the `OrbitCameraPlugin`, and add the `OrbitCamera` struct to the
//! entity containing the camera.
//!
//! For example, within the startup system:
//!
//! ```ignore
//! commands
//!     .spawn(Camera3dBundle {
//!         transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
//!             .looking_at(Vec3::default(), Vec3::unit_y()),
//!         ..Default::default()
//!     })
//!     .with(OrbitCamera::default());
//! ```
//!
//! To control the camera, use dragging (left button) to rotate and the mouse
//! wheel to zoom.

use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseScrollUnit::{Line, Pixel};
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

const LINE_TO_PIXEL_RATIO: f32 = 0.1;

pub struct OrbitCamera {
    pub x: f32,
    pub y: f32,
    pub distance: f32,
    pub center: Vec3,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub enabled: bool,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: 5.0,
            center: Vec3::ZERO,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
            enabled: true,
        }
    }
}

impl OrbitCamera {
    pub fn new(dist: f32, center: Vec3) -> OrbitCamera {
        OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: dist,
            center,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
            enabled: true,
        }
    }
}

pub struct OrbitCameraPlugin;
impl OrbitCameraPlugin {
    fn mouse_motion_system(
        time: Res<Time>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mouse_button_input: Res<Input<MouseButton>>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            delta += event.delta;
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            if !camera.enabled {
                continue;
            }
            if mouse_button_input.pressed(MouseButton::Left) {
                camera.x -= delta.x * camera.rotate_sensitivity * time.delta_seconds();
                camera.y -= delta.y * camera.rotate_sensitivity * time.delta_seconds();

                camera.y = camera.y.max(0.01).min(3.13);

                let rot = Quat::from_axis_angle(Vec3::Y, camera.x)
                    * Quat::from_axis_angle(-Vec3::X, camera.y);
                transform.translation =
                    (rot * Vec3::new(0.0, 1.0, 0.0)) * camera.distance + camera.center;
                transform.look_at(camera.center, Vec3::Y);
            }
        }
    }

    fn zoom_system(
        mut mouse_wheel_events: EventReader<MouseWheel>,
        mut query: Query<(&mut OrbitCamera, &mut Transform, &mut Camera)>,
    ) {
        let mut total = 0.0;
        for event in mouse_wheel_events.iter() {
            total += event.y
                * match event.unit {
                    Line => 1.0,
                    Pixel => LINE_TO_PIXEL_RATIO,
                };
        }
        for (mut camera, mut transform, _) in query.iter_mut() {
            if !camera.enabled {
                continue;
            }
            camera.distance *= camera.zoom_sensitivity.powf(total);
            let translation = &mut transform.translation;
            *translation =
                (*translation - camera.center).normalize() * camera.distance + camera.center;
        }
    }
}
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(Self::mouse_motion_system.system())
            .add_system(Self::zoom_system.system());
    }
}
