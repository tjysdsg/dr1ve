use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_scene_entities.system())
        .run();
}

struct CarEntity;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_xyz(1.5, 1.5, 1.5),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(3.0, 2.0, 3.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y()),
            ..Default::default()
        });

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -1.0),
        GlobalTransform::default(),
        CarEntity,
    ))
        .with_children(|parent| {
            parent.spawn_scene(asset_server.load("porsche911/scene.gltf#Scene0"));
        });
}

fn move_scene_entities(
    time: Res<Time>,
    mut scene_entities: Query<&mut Transform, With<CarEntity>>,
) {
    for mut transform in scene_entities.iter_mut() {
        transform.translation = Vec3::new(
            time.seconds_since_startup().sin() as f32 / 5.,
            0.,
            time.seconds_since_startup().cos() as f32 / 5.,
        );
    }
}
