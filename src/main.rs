use bevy::{
    core_pipeline::tonemapping::Tonemapping,  prelude::*, sprite::MaterialMesh2dBundle, window::{PrimaryWindow, WindowResized}
};

#[derive(Component)]
struct SpacePoint;

#[derive(Component)]
struct Jet;

#[derive(Component)]
struct Bullet;


#[derive(Resource)]
struct SpacePointTimer(Timer);

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

    commands.spawn((Jet,
        SpriteBundle {
        texture: asset_server.load("jet.png"),
        sprite: Sprite {
            color: Color::srgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
            custom_size: Some(Vec2::splat(160.0)),
            ..default()
        },
        ..default()
    }));

    // for i in -50..50 {
    //     for j in 0..50 {
    //         commands.spawn((
    //             SpacePoint,
    //             MaterialMesh2dBundle {
    //                 mesh: meshes.add(Circle::default()).into(),
    //                 material: materials.add(Color::srgb(7.5, 0.0, 7.5)),
    //                 transform: Transform::from_translation(Vec3::new(
    //                     (j * 20) as f32,
    //                     i as f32 * 20.,
    //                     0.,
    //                 )),
    //                 ..default()
    //             },
    //         ));
    //         commands.spawn((
    //             SpacePoint,
    //             MaterialMesh2dBundle {
    //                 mesh: meshes.add(Circle::default()).into(),
    //                 material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
    //                 transform: Transform::from_translation(Vec3::new(
    //                     (-1 * (j * 20)) as f32,
    //                     i as f32 * 20.,
    //                     0.,
    //                 )),
    //                 ..default()
    //             },
    //         ));
    //     }
    // }
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.read() {
        println!("width : {}", e.width);
        println!("height: {}", e.height);
    }
}

fn update_background(
    mut query: Query<&mut Transform, With<SpacePoint>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut timer: ResMut<SpacePointTimer>,
) {
    let window = window_query.get_single().unwrap();
    let window_height = window.height();

    if timer.0.tick(time.delta()).just_finished() {
        for mut transform in &mut query {
            transform.translation.y = transform.translation.y + 10.0;
            if transform.translation.y > window_height {
                transform.translation.y = 0. + (transform.translation.y - window_height);
            }
        }
    }
}

//ToDo: get the bounding box or height and then fire the bullet from there
fn udpate_on_button_click(mut query : Query<&Transform, With<Jet>>,
    // time : Res<Time>,
    // mut timer : ResMut<SpacePointTimer>,
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input : Res<ButtonInput<KeyCode>>){
    if keyboard_input.pressed(KeyCode::Space) {
       for tranform in &mut query {
            commands.spawn((
                SpacePoint,
                Bullet,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::srgb(255., 0.0, 7.5)),
                    transform: Transform::from_translation(Vec3::new(
                        tranform.translation.x,
                        tranform.translation.y,
                        0.
                    )),
                    ..default()
                },
            ));
        } 
        println!("space bar was pressed here");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpacePointTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_system)
        .add_systems(Update, on_resize_system)
        .add_systems(Update, (update_background,udpate_on_button_click))
        .run();
}
