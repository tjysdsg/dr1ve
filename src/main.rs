use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_scene(asset_server.load("porsche911/scene.gltf#Scene0"))
        .spawn(LightBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_xyz(7.0, 7.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::unit_y()),
            ..Default::default()
        });
}
