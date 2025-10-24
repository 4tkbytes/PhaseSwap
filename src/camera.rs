use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::math::{Quat, Vec2, Vec3};
use bevy::prelude::{Gamepad, GamepadAxis, Query, Res, Time, Transform, With, Without};
use crate::{Player, ThirdPersonCamera};

pub(crate) fn camera_controller(
    mut camera_query: Query<(&mut ThirdPersonCamera, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<ThirdPersonCamera>)>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
) {
    let Ok((mut camera, mut camera_transform)) = camera_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    camera.focus = player_transform.translation;

    if camera.is_locked {
        let mouse_delta = accumulated_mouse_motion.delta;

        if mouse_delta != Vec2::ZERO {
            camera.yaw -= mouse_delta.x * camera.sensitivity;
            camera.pitch -= mouse_delta.y * camera.sensitivity;
        }
    }

    for gamepad in gamepads.iter() {
        let right_stick_x = gamepad.get(GamepadAxis::RightStickX).unwrap_or(0.0);
        let right_stick_y = -gamepad.get(GamepadAxis::RightStickY).unwrap_or(0.0);

        const DEADZONE: f32 = 0.1;
        if right_stick_x.abs() > DEADZONE {
            camera.yaw += right_stick_x * camera.gamepad_sensitivity * time.delta_secs();
        }
        if right_stick_y.abs() > DEADZONE {
            camera.pitch -= right_stick_y * camera.gamepad_sensitivity * time.delta_secs();
        }
    }

    camera.pitch = camera.pitch.clamp(camera.min_pitch, camera.max_pitch);

    camera.yaw = camera.yaw.rem_euclid(std::f32::consts::TAU);

    let yaw_rotation = Quat::from_rotation_y(camera.yaw);
    let pitch_rotation = Quat::from_rotation_x(camera.pitch);

    let rotation = yaw_rotation * pitch_rotation;
    let offset = rotation * Vec3::new(0.0, 0.0, -camera.radius);

    camera_transform.translation = camera.focus + offset;
    camera_transform.look_at(camera.focus, Vec3::Y);
}