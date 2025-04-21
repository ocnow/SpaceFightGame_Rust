use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Assets},
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{
        default, in_state, Circle, Commands, Component, Entity, IntoSystemConfigs, KeyCode, Mesh,
        Mesh2d, NextState, OnEnter, Query, Res, ResMut, Resource, Transform, With,
    },
    sprite::{ColorMaterial, MeshMaterial2d, Sprite, SpriteBundle},
    time::{Time, Timer, TimerMode},
    window::{PrimaryWindow, Window},
};

use crate::{
    constants::{
        Bullet, BULLET_CIRCLE_RADIUS, BULLET_CREATE_TIMER_SECONDS, BULLET_VELOCITY,
        JET_SQUARE_BOX_LENGTH, JET_TRAVEL_DISTANCE,
    },
    GameState,
};

#[derive(Component)]
pub struct Jet;

#[derive(Component)]
pub struct GameEntity;

#[derive(Resource)]
struct BulletTimer(Timer);

pub struct JetPlugin;

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        GameEntity,
        Jet,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(JET_SQUARE_BOX_LENGTH)),
                image: asset_server.load("DurrrSpaceShip.png"),
                ..default()
            },
            ..default()
        },
    ));
}

fn udpate_on_button_click(
    mut query: Query<&mut Transform, With<Jet>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let window = window_query.get_single().unwrap();
    let window_height = window.height();
    let window_width = window.width();
    let mut jet_transform = query.single_mut();

    let (mut left, mut right) = (
        jet_transform.translation.x - (JET_SQUARE_BOX_LENGTH / 2.0),
        jet_transform.translation.x + (JET_SQUARE_BOX_LENGTH / 2.0),
    );
    let (mut top, mut bottom) = (
        jet_transform.translation.y + (JET_SQUARE_BOX_LENGTH / 2.0),
        jet_transform.translation.y - (JET_SQUARE_BOX_LENGTH / 2.0),
    );

    // println!("window height {}", window_height);
    if keyboard_input.pressed(KeyCode::KeyW) {
        top += JET_TRAVEL_DISTANCE;
        if top < window_height / 2. {
            jet_transform.translation.y += JET_TRAVEL_DISTANCE;
        }
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        bottom -= JET_TRAVEL_DISTANCE;
        if bottom > (-1. * window_height / 2.) {
            jet_transform.translation.y -= JET_TRAVEL_DISTANCE;
        }
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        left -= JET_TRAVEL_DISTANCE;
        if left > (-1. * window_width / 2.) {
            jet_transform.translation.x -= JET_TRAVEL_DISTANCE;
        }
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        right += JET_TRAVEL_DISTANCE;
        if right < window_width / 2. {
            jet_transform.translation.x += JET_TRAVEL_DISTANCE;
        }
    } else if keyboard_input.pressed(KeyCode::Escape) {
        game_state.set(GameState::Splash);
    }
}

fn create_bullets(
    mut jet_query: Query<&mut Transform, With<Jet>>,
    time: Res<Time>,
    mut bullet_timer: ResMut<BulletTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if bullet_timer.0.tick(time.delta()).just_finished() {
        for transform in &mut jet_query {
            commands.spawn((
                GameEntity,
                Bullet,
                Mesh2d(meshes.add(Circle::new(BULLET_CIRCLE_RADIUS))),
                MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 1.))),
                Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y + (JET_SQUARE_BOX_LENGTH / 2.),
                    0.,
                )),
            ));
        }
    }
}

fn update_bullets(
    mut query: Query<(&mut Transform, Entity), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let window_height = window.height();

    for (mut transform, entity) in &mut query {
        transform.translation.y = transform.translation.y + BULLET_VELOCITY;
        if transform.translation.y > window_height / 2. {
            commands.entity(entity).despawn();
        }
    }
}

impl Plugin for JetPlugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.insert_resource(BulletTimer(Timer::from_seconds(
            BULLET_CREATE_TIMER_SECONDS,
            TimerMode::Repeating,
        )));
        app.add_systems(OnEnter(GameState::Game), setup_system);
        app.add_systems(
            Update,
            (
                udpate_on_button_click,
                (create_bullets, update_bullets).chain(),
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}
