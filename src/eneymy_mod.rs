use bevy::{
    a11y::accesskit::Size,
    app::{App, Plugin, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    math::{
        bounding::{Aabb2d, BoundingCircle},
        Vec2, Vec3,
    },
    prelude::{
        default, Bundle, Commands, Component, Deref, Entity, EventReader, EventWriter, Image,
        Query, Res, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    time::Time,
    window::{PrimaryWindow, Window},
};
use rand::{thread_rng, Rng};

use crate::{
    constants::{
        Bullet, CollisionEvent, CollisionSound, ENEMY_BULLET_STRUCK_EVENT, ENEMY_DIED_EVENT,
        ENEMY_SQUARE_BOX_LENGTH,
    },
    utils::ball_collision,
};

pub struct EnemyPlugin;

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref)]
struct XP(i32);

#[derive(Bundle)]
struct EnemyObjectBundle {
    xp: XP,
    sprite: SpriteBundle,
}

fn create_space_enemy_objects(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn(get_enemy_bundle(
        asset_server.load("space_object.png"),
        window_width,
        window_height,
    ));
}

fn get_enemy_bundle(image_handle: Handle<Image>, width: f32, height: f32) -> impl Bundle {
    let mut rng = thread_rng();
    let x = rng.gen_range((-1. * width / 2.) + (ENEMY_SQUARE_BOX_LENGTH/2.0)..(width / 2.) - (ENEMY_SQUARE_BOX_LENGTH/2.0));
    let y = rng.gen_range(0.0..(height / 2.) - (ENEMY_SQUARE_BOX_LENGTH/2.0));
    return (
        Enemy,
        EnemyObjectBundle {
            xp: XP(20),
            sprite: SpriteBundle {
                texture: image_handle,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(ENEMY_SQUARE_BOX_LENGTH)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(0.5, 0.5, 1.0),
                    ..default()
                },
                ..default()
            },
        },
    );
}

fn check_for_collision(
    mut enemy_object: Query<(Entity,&Transform, &Sprite, &mut XP), With<Enemy>>,
    mut bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
    mut collision_events: EventWriter<CollisionEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (enemy_entity,enemy_object_transform, sprite, mut xp) = enemy_object.single_mut();
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

                commands.spawn(get_enemy_bundle(
                    asset_server.load("space_object.png"),
                    window_width,
                    window_height,
                ));
            }
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.add_event::<CollisionEvent>();
        app.add_systems(Startup, create_space_enemy_objects);
        app.add_systems(Update, check_for_collision);
    }
}
