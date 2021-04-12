# `bevy-orbit-controls`

An orbit controls plugin for bevy.

To control the camera, drag the mouse. The left button rotates. The
wheel zooms.

## Usage

Register the `OrbitCameraPlugin`, and insert the `OrbitCamera` struct
into the entity containing the camera.

For example, within the startup system:

```rust
commands
    .spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(-3.0, 3.0, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    })
    .insert(OrbitCamera::default());
```

## Compatibility

- `v2.x` – Bevy `0.5`.
- `v1.x` – Bevy `0.4`.
