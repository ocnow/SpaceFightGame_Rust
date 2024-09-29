use bevy::{
    app::{FixedUpdate, Plugin, Startup, Update},
    asset::{AssetServer, Assets},
    color::Color,
    math::Vec3,
    prelude::{
        default, App, Bundle, Circle, Commands, Component, Mesh, Query, Res, ResMut, Resource,
        Transform, With,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

pub struct SpacePointPlugin;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct XP(i32);

#[derive(Bundle)]
struct EnemyObjectBundle {
    xp: XP,
    sprite: SpriteBundle,
}

#[derive(Resource)]
struct SpacePointTimer(Timer);

#[derive(Component)]
struct SpacePoint;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in -50..50 {
        for j in 0..50 {
            commands.spawn((
                SpacePoint,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::srgb(7.5, 0.0, 7.5)),
                    transform: Transform::from_translation(Vec3::new(
                        (j * 20) as f32,
                        i as f32 * 20.,
                        0.,
                    )),
                    ..default()
                },
            ));
            commands.spawn((
                SpacePoint,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::default()).into(),
                    material: materials.add(Color::srgb(7.5, 0.0, 7.5)),
                    transform: Transform::from_translation(Vec3::new(
                        (-1 * (j * 20)) as f32,
                        i as f32 * 20.,
                        0.,
                    )),
                    ..default()
                },
            ));
        }
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

// Implement methods or traits for the struct
impl Plugin for SpacePointPlugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.insert_resource(SpacePointTimer(Timer::from_seconds(
            5.,
            TimerMode::Repeating,
        )));
        app.add_systems(Startup, setup_system);
        app.add_systems(Update, update_background);
    }
}
