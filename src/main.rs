use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(
                Window {
                    title: "PhaseSwap".to_string(),
                    ..default()
                }
            ),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            camera_controller,
        ).chain())
        .run();
}

#[derive(Component)]
struct Player {
    speed: f32,
    rotation_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            rotation_speed: 10.0,
        }
    }
}

#[derive(Component)]
#[allow(dead_code)]
struct ThirdPersonCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub min_pitch: f32,
    pub max_pitch: f32,
    pub min_radius: f32,
    pub max_radius: f32,
    pub zoom_speed: f32,
    pub is_locked: bool,
}

impl Default for ThirdPersonCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 8.0,
            yaw: 0.0,
            pitch: 20.0_f32.to_radians(),
            sensitivity: 0.002,
            gamepad_sensitivity: 2.0,
            min_pitch: (-80.0_f32).to_radians(),
            max_pitch: 80.0_f32.to_radians(),
            min_radius: 5.0,
            max_radius: 20.0,
            zoom_speed: 1.0,
            is_locked: true,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player
    commands.spawn((
        Player::default(),
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // cube for ref
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        ThirdPersonCamera::default(),
    ));

    // baseplate
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
}

fn player_movement(
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

fn camera_controller(
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