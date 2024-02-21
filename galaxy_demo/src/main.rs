use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup_cam, add_stars))
    .run();
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default()
    )); // Bundle trait enables insertion and removal of Components from an entity. Here we add the PanOrbitCamera trait to the default 3D camera.
}


fn add_stars(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = mesh_assets.add(Sphere::default().mesh()); // add a new mesh
    
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(
                    StandardMaterial {
                        base_color: Color::RED,
                        ..default()
                    }),
                transform: Transform::from_translation(Vec3::new(x as f32, 0., z as f32)),
                ..default() // default value spread
            });
        }
    }
    
}