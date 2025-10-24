use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use crate::{Player, ThirdPersonCamera};

#[derive(Resource)]
pub struct PlayerMeshes {
    pub(crate) cube: Handle<Mesh>,
    pub(crate) sphere: Handle<Mesh>,
    pub(crate) cylinder: Handle<Mesh>,
}

pub(crate) fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    mut camera_query: Query<&mut ThirdPersonCamera>,
    mut cursor_options: Single<&mut CursorOptions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    time: Res<Time>,
) {
    let Ok((player, mut transform)) = player_query.single_mut() else {
        return;
    };

    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    if camera.is_locked {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    } else {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    }

    let mut input = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::F1) {
        camera.is_locked = !camera.is_locked;
    } else if keyboard_input.just_released(KeyCode::F1) {
        camera.is_locked = camera.is_locked;
    }

    for gamepad in gamepads.iter() {
        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let left_stick_y = -gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

        const DEADZONE: f32 = 0.1;
        if left_stick_x.abs() > DEADZONE {
            input.x -= left_stick_x;
        }
        if left_stick_y.abs() > DEADZONE {
            input.y += left_stick_y;
        }
    }


    if input.length() > 1.0 {
        input = input.normalize();
    }

    if input.length() > 0.01 {
        let camera_forward = Vec3::new(
            camera.yaw.sin(),
            0.0,
            camera.yaw.cos(),
        );

        let camera_right = Vec3::new(
            camera.yaw.cos(),
            0.0,
            -camera.yaw.sin(),
        );

        let move_direction = (camera_forward * -input.y + camera_right * input.x).normalize();

        let movement = move_direction * player.speed * time.delta_secs();
        transform.translation += movement;

        let target_rotation = Quat::from_rotation_y(
            -move_direction.x.atan2(-move_direction.z)
        );
        transform.rotation = transform.rotation.slerp(
            target_rotation,
            player.rotation_speed * time.delta_secs()
        );
    }
}

pub(crate) fn switch_player_mesh(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_meshes: Res<PlayerMeshes>,
    mut query: Query<&mut Mesh3d, With<Player>>,
) {
    if let Ok(mut mesh) = query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            mesh.0 = player_meshes.cube.clone();
        } else if keyboard_input.just_pressed(KeyCode::Digit2) {
            mesh.0 = player_meshes.sphere.clone();
        } else if keyboard_input.just_pressed(KeyCode::Digit3) {
            mesh.0 = player_meshes.cylinder.clone();
        }
    }
}

