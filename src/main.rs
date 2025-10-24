mod camera;
mod player;

use bevy::prelude::*;
use crate::camera::camera_controller;
use crate::player::{player_movement, switch_player_mesh, PlayerMeshes};

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
            switch_player_mesh
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
    let player_meshes = PlayerMeshes {
        cube: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        sphere: meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap()),
        cylinder: meshes.add(Cylinder::new(0.5, 1.0)),
    };

    // player
    commands.spawn((
        Mesh3d(player_meshes.cube.clone()),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player::default(),
    ));
    commands.insert_resource(player_meshes);

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