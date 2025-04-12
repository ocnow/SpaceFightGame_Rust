use bevy::{
    asset::Handle,
    audio::AudioSource,
    math::Vec2,
    prelude::{Component, Deref, Event, Resource},
};

pub const JET_SQUARE_BOX_LENGTH: f32 = 50.0;
pub const BULLET_CREATE_TIMER_SECONDS: f32 = 0.5;
pub const BULLET_CIRCLE_RADIUS: f32 = 5.0;
pub const BULLET_VELOCITY: f32 = 5.0;
pub const JET_TRAVEL_DISTANCE: f32 = 5.0;
pub const ENEMY_SQUARE_BOX_LENGTH: f32 = 100.0;
pub const ENEMY_SPAWN_HEALTH: i32 = 20;
pub const ENEMY_SPACE_SPRITE_NAME: &str = "spaceship_small.png";
pub const ENEMY_OBJECT_SCALE: Vec2 = Vec2::new(1., 1.);

#[derive(Component)]
pub struct Bullet;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Resource, Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);
