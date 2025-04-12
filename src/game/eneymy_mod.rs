use std::usize;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    image,
    math::{
        bounding::{Aabb2d, BoundingCircle},
        Vec2, Vec3,
    },
    prelude::{
        default, in_state, Bundle, Circle, Commands, Component, Deref, Entity, EventWriter, Image,
        IntoSystemConfigs, Mesh, Mesh2d, OnEnter, Query, Rectangle, Res, ResMut, Single, Text,
        TextUiWriter, Transform, With,
    },
    sprite::{ColorMaterial, MeshMaterial2d, Sprite},
    text::{Text2dWriter, TextSpan},
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use crate::{
    constants::{
        Bullet, CollisionEvent, ENEMY_OBJECT_SCALE, ENEMY_SPACE_SPRITE_NAME, ENEMY_SPAWN_HEALTH,
        ENEMY_SQUARE_BOX_LENGTH,
    },
    utils::ball_collision,
    GameState,
};

use super::{
    player_jet_mod::{GameEntity, Jet},
    LevelText, Score,
};

pub struct EnemyPlugin;

#[derive(Component)]
struct Enemy;

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
    let window = window_query.get_single().unwrap();
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
    ));
}

fn get_enemy_bundle(image_handle: Handle<Image>, width: f32, height: f32) -> impl Bundle {
    let mut rng = thread_rng();
    let x = rng.gen_range(
        (-1. * width / 2.) + (ENEMY_SQUARE_BOX_LENGTH / 2.0)
            ..(width / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0),
    );
    let y = rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0));
    return (
        GameEntity,
        Enemy,
        EnemyObjectBundle {
            xp: XP(ENEMY_SPAWN_HEALTH),
            // sprite: SpriteBundle {
            //     texture: image_handle,
            //     sprite: Sprite {
            //         custom_size: Some(Vec2::splat(ENEMY_SQUARE_BOX_LENGTH)),
            //         ..default()
            //     },
            //     transform: Transform {
            //         translation: Vec3::new(x, y, 0.),
            //         scale: Vec3::new(0.5, 0.5, 1.0),
            //         ..default()
            //     },
            //     ..default()
            // },
            sprite: Sprite {
                image: image_handle,
                // custom_size: Some(Vec2::splat(ENEMY_SQUARE_BOX_LENGTH)),
                ..default()
            },
        },
    );
}

fn check_for_collision(
    mut enemy_object: Query<(Entity, &Transform, &Sprite, &mut XP), With<Enemy>>,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
    mut collision_events: EventWriter<CollisionEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
    score_root: Single<Entity, (With<LevelText>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let (enemy_entity, enemy_object_transform, sprite, mut xp) = enemy_object.single_mut();
    let image_dimensions = sprite.custom_size.unwrap();
    let image_dims_in_vec2 = Vec2::new(image_dimensions.x as f32, image_dimensions.y as f32);
    let scaled_image_dimension = image_dims_in_vec2 * enemy_object_transform.scale.truncate();
    // let bounding_box = Rect::from_center_size(enemy_object_transform.translation.truncate(), scaled_image_dimension);
    for (bullet_entity, bullet_transform) in &mut bullets {
        // println!("bullet found:{}",enemy_object_transform.translation.truncate());
        let collision = ball_collision(
            BoundingCircle::new(bullet_transform.translation.truncate(), 5.),
            Aabb2d::new(
                enemy_object_transform.translation.truncate(),
                scaled_image_dimension / 2.,
            ),
        );

        if collision {
            println!("collision happened");
            commands.entity(bullet_entity).despawn();
            collision_events.send_default();
            xp.0 = xp.0 - 1;
            if xp.0 == 0 {
                let window = window_query.get_single().unwrap();
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
                let y = rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH / 2.0));

                commands.spawn((
                    get_enemy_bundle(
                        asset_server.load(ENEMY_SPACE_SPRITE_NAME),
                        window_width,
                        window_height,
                    ),
                    Transform {
                        translation: Vec3::new(x, y, 0.),
                        scale: Vec3::new(0.5, 0.5, 1.0),
                        ..default()
                    },
                ));

                score.0 += 1;
                *writer.text(*score_root, 1) = score.0.to_string();
                // // for mut text in text_query.iter_mut() {
                // //     text.sections[0].value = format!("Score {}", score.0);
                // //     text.text
                // // }
                // for mut text_entity in text_query.iter_mut() {
                //     println!("value found in query");
                //     **text_entity = format!("Score {}", score.0);
                // }
                println!("now just updating the score value");
            }
        }
    }
}

// fn check_for_collision2(
//     mut commands: Commands,
//     mut enemy_object: Query<(Entity, &Transform, &Sprite, &mut XP), With<Enemy>>,
//     mut bullets: Query<(Entity, &Transform), With<Bullet>>,
//     mut collision_events: EventWriter<CollisionEvent>,
//     mut image_asset: ResMut<Assets<Image>>,
//     mut score: ResMut<Score>,
// ) {
//     let (enemy_entity, enemy_object_transform, sprite, mut xp) = enemy_object.single_mut();
//     let image_ref = &sprite.image;
//     println!("transform : {}", enemy_object_transform.translation);
//     let image_handle = image_asset.get(image_ref);
//     let image_size = image_handle.size_f32();
//     for (bullet_entity, bullet_transform) in &mut bullets {
//         let translation = bullet_transform.translation - enemy_object_transform.translation;
//
//         let x = (translation.x + image_size.x / 2.0) as usize;
//         let y = (translation.y + image_size.y / 2.0) as usize;
//
//         if x < image_size.x as usize && y < image_size.y as usize {
//             let idx = (y * image_size.x as usize + x) * 4;
//             let alpha = image_handle.data[idx + 3];
//             if alpha > 0 {
//                 println!("collision happened");
//                 commands.entity(bullet_entity).despawn();
//                 collision_events.send_default();
//                 xp.0 = xp.0 - 1;
//             }
//         }
//     }
// }

fn check_for_collision_3(
    mut enemy_object_query: Query<(Entity, &Transform, &Sprite, &mut XP), With<Enemy>>,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut collision_events: EventWriter<CollisionEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
    score_root: Single<Entity, (With<LevelText>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    for (bullet_entity, bullet_transform) in &mut bullets {
        // println!("bullet pos:{}", bullet_transform.translation);
        for (enemy_entity, enemy_object_transform, sprite, mut xp) in &mut enemy_object_query {
            let image_ref = &sprite.image;
            if let Some(enemy_image) = images.get(image_ref) {
                let enemy_size_f32 = enemy_image.size_f32();

                // println!("size of enemy {},{}", enemy_size_f32.x, enemy_size_f32.y);
                let bullet_center_x = bullet_transform.translation.x;
                let bullet_center_y = bullet_transform.translation.y;

                let enemy_center_x = enemy_object_transform.translation.x;
                let enemy_center_y = enemy_object_transform.translation.y;

                //check for collision
                //enemy_x - size/2 < pos < enemy_x + size/2
                //enemy_y - size /2 > pos

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
                    // commands.spawn((
                    //     // MeshMaterial2d {
                    //     //     mesh: meshes.add(Circle::new(BULLET_CIRCLE_RADIUS)).into(),
                    //     //     material: materials.add(Color::srgb(0.5, 0.5, 1.)),
                    //     //     transform: Transform::from_translation(Vec3::new(
                    //     //         transform.translation.x,
                    //     //         transform.translation.y + (JET_SQUARE_BOX_LENGTH / 2.),
                    //     //         0.,
                    //     //     )),
                    //     //     ..default()
                    //     // },
                    //     Mesh2d(meshes.add(Rectangle::new(enemy_size_f32.x, enemy_size_f32.y))),
                    //     MeshMaterial2d(materials.add(Color::srgb(0.99, 0.0, 1.))),
                    //     Transform::from_translation(Vec3::new(enemy_center_x, enemy_center_y, 0.)),
                    // ));

                    let pos_y = top_left_enemy_y - bullet_center_y;
                    let pos_x = (top_left_enemy_x - bullet_center_x).abs();

                    // println!("position in square:{},{}", pos_x, pos_y);
                    let idx = (pos_y as usize * enemy_size_f32.x as usize + pos_x as usize) * 4;
                    let alpha = enemy_image.data[idx as usize];
                    // println!(
                    //     "collision happened pixel cords {},{},{}",
                    //     pos_x, pos_y, alpha
                    // );
                    if alpha > 0 {
                        println!("collision happened");
                        commands.entity(bullet_entity).despawn();
                        collision_events.send_default();
                        xp.0 = xp.0 - 1;
                        println!("xp now is {}", xp.0);
                        if xp.0 == 0 {
                            let window = window_query.get_single().unwrap();
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
                            // // for mut text in text_query.iter_mut() {
                            // //     text.sections[0].value = format!("Score {}", score.0);
                            // //     text.text
                            // // }
                            // for mut text_entity in text_query.iter_mut() {
                            //     println!("value found in query");
                            //     **text_entity = format!("Score {}", score.0);
                            // }
                            println!("now just updating the score value");
                        }
                    } else {
                        // println!("cool");
                    }
                    // let x = (translation.x + enemy_size_f32.x / 2.0) as usize;
                    // let y = (translation.y + enemy_size_f32.y / 2.0) as usize;
                    //
                    //     let idx = (y * enemy_size_f32.x as usize + x) * 4;
                    //     let alpha = enemy_image.data[idx];
                    //
                    //     if alpha > 0 {
                    //         println!("pixel perfect hit");
                    //         commands.entity(bullet_entity).despawn();
                    //     }
                    // }
                    // println!("here we go");
                    // println!("{:?}", enemy_image.texture_descriptor.format);
                    //
                    // println!("image.data.len(): {}", enemy_image.data.len());
                    // println!(
                    //     "expected size: {}",
                    //     enemy_image.size_f32().x * enemy_image.size_f32().y * 4.0
                    // );
                    // for y in 0..5 {
                    //     for x in 0..5 {
                    //         let idx = (y * enemy_image.size().x as usize + x) * 4;
                    //         println!("Pixel ({},{}) alpha: {}", x, y, enemy_image.data[idx + 3]);
                    //     }
                    // }
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
        app.add_systems(
            Update,
            check_for_collision_3.run_if(in_state(GameState::Game)),
        );
        // app.add_systems(
        //     Update,
        //     check_for_collision.run_if(in_state(GameState::Game)),
        // );
    }
}
