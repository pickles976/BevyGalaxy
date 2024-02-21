use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup_cam, spawn_cubes))
    .add_systems(Update, basic_system)
    .run();
}

fn basic_system(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        debug!("Space was pressed");
    }
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle::default()); // Bundle trait enables insertion and removal of Components from an entity. Here we add the FlyCam trait to the default 3D camera.
}

fn spawn_cubes(
    mut commands: Commands, // Commands is a queue to perform structural changes to the World
    mut mesh_assets: ResMut<Assets<Mesh>>, // Assets is kind of like a store that we can read from and add to, hence the ResMut wrapper
) {
    let mesh = mesh_assets.add(Cuboid::new(1., 1., 1.));
    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_translation(Vec3::new(x as f32 * 2., 0., z as f32 * 2.)),
                ..Default::default()
            });
        }
    }
}