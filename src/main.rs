use bevy::{
    a11y::accesskit::Rect, core_pipeline::tonemapping::Tonemapping, math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*, render::view::window, sprite::MaterialMesh2dBundle, window::{PrimaryWindow, WindowResized}
};
use rand::{thread_rng, Rng};

#[derive(Component)]
struct SpacePoint;

#[derive(Component)]
struct Jet;

#[derive(Component)]
struct Bullet;

#[derive(Resource)]
struct SpacePointTimer(Timer);

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct BulletTimer(Timer);

#[derive(Component)]
struct XP(i32);

#[derive(Bundle)]
struct EnemyObjectBundle {
    xp: XP,
    sprite: SpriteBundle,
}

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

    commands.spawn((
        Jet,
        SpriteBundle {
            texture: asset_server.load("jet.png"),
            sprite: Sprite {
                color: Color::srgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
                custom_size: Some(Vec2::splat(50.0)),
                ..default()
            },
            ..default()
        },
    ));

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
        // println!("width : {}", e.width);
        // println!("height: {}", e.height);
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
            if transform.translation.y > window_height / 2. {
                transform.translation.y = 0. + (transform.translation.y - window_height / 2.);
            }
        }
    }
}

fn update_bullets(
    mut query: Query<(&mut Transform, Entity), With<Bullet>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut timer: ResMut<SpacePointTimer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let window_height = window.height();

    //logic to make despawn bullets out of window
    if timer.0.tick(time.delta()).just_finished() {
        for (mut transform, entity) in &mut query {
            transform.translation.y = transform.translation.y + 10.0;
            if transform.translation.y > window_height / 2. {
                commands.entity(entity).despawn();
            }
        }
    }

    //
}

fn create_bullets(
    mut jet_query: Query<&mut Transform, With<Jet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut bullet_timer: ResMut<BulletTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        for transform in &mut jet_query {
            commands.spawn((
                Bullet,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(5.)).into(),
                    material: materials.add(Color::srgb(0.5, 0.5, 1.)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y + 25.0,
                        0.,
                    )),
                    ..default()
                },
            ));
        }
    }
}

fn create_space_enemy_objects(
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.width();

    let mut rng = thread_rng();
    let x = rng.gen_range((-1. * window_width / 2.) - 25.0..(window_width / 2.) + 25.);
    let y = 0.;

    // println!("enemy object :{}{}",x,y);
    // commands.spawn((Jet,
    //     SpriteBundle {
    //     texture: asset_server.load("space_object.png"),
    //     sprite: Sprite {
    //         color: Color::srgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
    //         custom_size: Some(Vec2::splat(50.0)),
    //         ..default()
    //     },
    //     ..default()
    // }));
    // 
    println!("spawning at:{}",x);

    commands.spawn((
        Enemy,
        EnemyObjectBundle {
        xp: XP(20),
        sprite: SpriteBundle {
            texture: asset_server.load("space_object.png"),
            sprite: Sprite {
                color: Color::srgb(5.0, 5.0, 5.0), // 4. Put something bright in a dark environment to see the effect
                // custom_size: Some(Vec2::splat(50.)),
                ..default()
            },
                transform: Transform {
                        translation: Vec3::new(x,y,0.),
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..default()
                    },
            ..default()
        },
    },));
// // // brick
//             commands.spawn((
//                 SpriteBundle {
//                     sprite: Sprite {
//                         color:Color::srgb(5.0, 5.0, 5.0),
//                         ..default()
//                     },
//                     transform: Transform {
//                         translation: Vec3::new(x, y, 0.),
//                         scale: Vec3::new(100., 30., 1.0),
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 Enemy
//             ));
}

//ToDo: get the bounding box or height and then fire the bullet from there
fn udpate_on_button_click(
    mut query: Query<&mut Transform, With<Jet>>,
    // time : Res<Time>,
    // mut timer : ResMut<SpacePointTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_height = window.height();
    let window_width = window.width();

    // println!("window height {}", window_height);
    if keyboard_input.pressed(KeyCode::KeyW) {
        for mut transform in &mut query {
            transform.translation.y = transform.translation.y + 20.0;
            if transform.translation.y > window_height / 2. {
                transform.translation.y = window_height / 2.;
            }
        }
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        for mut transform in &mut query {
            transform.translation.y = transform.translation.y - 20.0;
            if transform.translation.y < (-1. * window_height / 2.) {
                transform.translation.y = -1. * window_height / 2.;
            }
        }
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        for mut transform in &mut query {
            transform.translation.x = transform.translation.x - 20.0;
            if transform.translation.x < (-1. * window_width / 2.) {
                transform.translation.x = -1. * window_width / 2.;
            }
        }
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        for mut transform in &mut query {
            transform.translation.x = transform.translation.x + 20.0;
            if transform.translation.x > window_width / 2. {
                transform.translation.x = window_width / 2.;
            }
        }
    }
    // for transform in &query{
    //     println!("{}",transform.translation.x);
    //     println!("{}",transform.translation.y);
    // }
}

fn check_for_collision(
    mut enemyObject : Query<(Entity,&Transform,&Handle<Image>),With<Enemy>>,
    mut bullets : Query<(Entity,&Transform),With<Bullet>>,
assets: Res<Assets<Image>>,
) {
    let (mut enemy_object_entity,enemy_object_transform,image_handle) = enemyObject.single_mut();
         let image_dimensions = assets.get(image_handle).unwrap().size();
        let image_dims_in_vec2 = Vec2::new(image_dimensions.x as f32, image_dimensions.y as f32);
        let scaled_image_dimension = image_dims_in_vec2 * enemy_object_transform.scale.truncate();
        // let bounding_box = Rect::from_center_size(enemy_object_transform.translation.truncate(), scaled_image_dimension);
    for (bullet_entity,bullet_transform) in &mut bullets {
       // println!("bullet found:{}",enemy_object_transform.translation.truncate());
        let collision = ball_collision(
            BoundingCircle::new(bullet_transform.translation.truncate(), 5.),
        Aabb2d::new(
                enemy_object_transform.translation.truncate(),
               scaled_image_dimension/ 2.,
            ));

        if(collision){
            println!("collision happened");
        }
    }
       println!("bullet found:{}",enemy_object_transform.translation.truncate());
    println!("==========================================================");
}

fn ball_collision(bullet_circle : BoundingCircle, 
        bounding_box : Aabb2d
) -> bool {
    if bullet_circle.intersects(&bounding_box) {
       return true; 
    }
    return false;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SpacePointTimer(Timer::from_seconds(
            0.01,
            TimerMode::Repeating,
        )))
        .insert_resource(BulletTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (setup_system,
                create_space_enemy_objects,)
        )
        .add_systems(Update, on_resize_system)
        .add_systems(
            Update,
            (
                update_background,
                udpate_on_button_click,
            ).chain(),
        )
        .add_systems(FixedUpdate, 
            (create_bullets,
                update_bullets,
                check_for_collision,
            ).chain()
        )
        .run();
}
