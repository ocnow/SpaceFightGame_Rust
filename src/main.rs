use bevy::{
    core_pipeline::tonemapping::Tonemapping, prelude::*, sprite::MaterialMesh2dBundle,
    window::WindowResized,
};

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            hdr: true, // 1. HDR is required for bloom
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        ..default()
    },));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("jet.png"),
        sprite: Sprite {
            color: Color::rgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
            custom_size: Some(Vec2::splat(160.0)),
            ..default()
        },
        ..default()
    });

    for i in -80..80 {
        for j in 0..80 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Capsule2d::new(5.0, 5.0)).into(),
                material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
                transform: Transform::from_translation(Vec3::new((j * 50) as f32, i as f32 * 50., 0.)),
                ..default()
            });
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Capsule2d::new(5.0, 5.0)).into(),
                material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
                transform: Transform::from_translation(Vec3::new(
                    (-1 * (j * 50)) as f32,
                    i as f32 * 50.0,
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        println!("width : {}", e.width);
        println!("height: {}", e.height);
    }
}

fn update_system(mut query : Query<>) {
    println!("hello this is update system")
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_system)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, update_system)
        .run();
}
