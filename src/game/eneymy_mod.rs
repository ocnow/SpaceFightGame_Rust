use std::usize;
use bevy::prelude::*;

use bevy::{
    app::{App, Plugin, Update}, asset::{AssetServer, Assets, Handle}, ecs::{schedule::IntoScheduleConfigs, system::SystemParamFunction}, math::Vec3, prelude::{
        default, in_state, Bundle, Commands, Component, Deref, Entity, EventWriter, Image,
         OnEnter, Query, Res, ResMut, Single, Text,
        TextUiWriter, Transform, With,
    }, sprite::Sprite, window::{PrimaryWindow, Window}
};
use rand::{thread_rng, Rng};

use crate::{
    constants::{
        Bullet, CollisionEvent, ENEMY_OBJECT_SCALE, ENEMY_SPACE_SPRITE_NAME, ENEMY_SPAWN_HEALTH,
        ENEMY_SQUARE_BOX_LENGTH,
    },
    GameState,
};

use super::player_jet_mod::observe_bullet_event;
use super::{
    player_jet_mod::{GameEntity, SpaceShip},
    LevelText, Score,
};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref)]
struct XP(i32);

#[derive(Bundle)]
struct EnemyObjectBundle {
    xp: XP,
    sprite: Sprite,
}

fn create_space_enemy_objects(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    let width = window_width;
    let height = window_height;
    let mut rng = thread_rng();
    let x = rng.gen_range(
        (-1. * width / 2.) + (ENEMY_SQUARE_BOX_LENGTH / 2.0)
            ..(width / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0),
    );
    let y = rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0));
    commands.spawn((
        get_enemy_bundle(
            asset_server.load(ENEMY_SPACE_SPRITE_NAME),
            window_width,
            window_height,
        ),
        Transform {
            translation: Vec3::new(x, y, 0.),
            scale: ENEMY_OBJECT_SCALE.extend(1.),
            ..default()
        },
    )).observe(observe_bullet_event);
}

fn get_enemy_bundle(image_handle: Handle<Image>, width: f32, height: f32) -> impl Bundle {
    let mut rng = thread_rng();
    let x = rng.gen_range(
        (-1. * width / 2.) + (ENEMY_SQUARE_BOX_LENGTH / 2.0)
            ..(width / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0),
    );
    let y = rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0));
    (
        GameEntity,
        Enemy,
        SpaceShip,
        EnemyObjectBundle {
            xp: XP(ENEMY_SPAWN_HEALTH),
            sprite: Sprite {
                image: image_handle,
                ..default()
            },
        },
    )
}

fn check_for_collision(
    mut enemy_object_query: Query<(Entity, &Transform, &Sprite, &mut XP), With<Enemy>>,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
    mut collision_events: EventWriter<CollisionEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
    score_root: Single<Entity, (With<LevelText>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    for (bullet_entity, bullet_transform) in &mut bullets {
        for (enemy_entity, enemy_object_transform, sprite, mut xp) in &mut enemy_object_query {
            let image_ref = &sprite.image;
            if let Some(enemy_image) = images.get(image_ref) {
                let enemy_size_f32 = enemy_image.size_f32();

                // println!("size of enemy {},{}", enemy_size_f32.x, enemy_size_f32.y);
                let bullet_center_x = bullet_transform.translation.x;
                let bullet_center_y = bullet_transform.translation.y;

                let enemy_center_x = enemy_object_transform.translation.x;
                let enemy_center_y = enemy_object_transform.translation.y;

                let smallest_x_of_enemy = enemy_center_x - (enemy_size_f32.x / 2.);
                let highest_x_of_enemy = enemy_center_x + (enemy_size_f32.x / 2.);
                let smallest_y_of_enemy = enemy_center_y - (enemy_size_f32.y / 2.);
                let highest_y_of_enemy = enemy_center_y + (enemy_size_f32.y / 2.);
                //
                if smallest_x_of_enemy < bullet_center_x
                    && highest_x_of_enemy > bullet_center_x
                    && smallest_y_of_enemy < bullet_center_y
                {
                    let top_left_enemy_x = smallest_x_of_enemy;
                    let top_left_enemy_y = highest_y_of_enemy;

                    let pos_y = top_left_enemy_y - bullet_center_y;
                    let pos_x = (top_left_enemy_x - bullet_center_x).abs();

                    let idx = (pos_y as usize * enemy_size_f32.x as usize + pos_x as usize) * 4;
                    let alpha = enemy_image.data.clone().unwrap();
                    let alpha = alpha[idx];
                    if alpha > 0 {
                        println!("collision happened");
                        commands.entity(bullet_entity).despawn();
                        collision_events.write_default();
                        xp.0 -= 1;
                        println!("xp now is {}", xp.0);
                        if xp.0 == 0 {
                            let window = window_query.single().unwrap();
                            let window_width = window.width();
                            let window_height = window.height();

                            commands.entity(enemy_entity).despawn();
                            let width = window_width;
                            let height = window_height;

                            let mut rng = thread_rng();
                            let x = rng.gen_range(
                                (-1. * width / 2.) + (ENEMY_SQUARE_BOX_LENGTH / 2.0)
                                    ..(width / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0),
                            );
                            let y =
                                rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0));

                            commands.spawn((
                                get_enemy_bundle(
                                    asset_server.load(ENEMY_SPACE_SPRITE_NAME),
                                    window_width,
                                    window_height,
                                ),
                                Transform {
                                    translation: Vec3::new(x, y, 0.),
                                    ..default()
                                },
                            ));

                            score.0 += 1;
                            *writer.text(*score_root, 1) = score.0.to_string();
                            println!("now just updating the score value");
                        }
                    }
                }
            }
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.add_event::<CollisionEvent>();
        app.add_systems(OnEnter(GameState::Game), create_space_enemy_objects);
        // app.add_systems(
        //     Update,
        //     check_for_collision.run_if(in_state(GameState
        //             ::Game))
        // );
    }
}
