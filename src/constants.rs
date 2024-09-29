

use bevy::{asset::Handle, audio::AudioSource, prelude::{default, Component, Deref, Event, Resource}};

pub const JET_SQUARE_BOX_LENGTH : f32 = 50.0;
pub const BULLET_CREATE_TIMER_SECONDS : f32 = 0.1;
pub const BULLET_CIRCLE_RADIUS : f32 = 5.0;
pub const BULLET_VELOCITY : f32 =  5.0;
pub const JET_TRAVEL_DISTANCE : f32 = 5.0;
pub const ENEMY_SQUARE_BOX_LENGTH : f32 = 200.0;

pub const ENEMY_BULLET_STRUCK_EVENT : &str = "ENEMY_BULLETE_STRUCK";
pub const ENEMY_DIED_EVENT : &str = "ENEMY_DIED_EVENT";


#[derive(Component)]
pub struct Bullet;

#[derive(Event,Default)]
pub struct CollisionEvent;

#[derive(Resource,Deref)]
pub struct CollisionSound(pub Handle<AudioSource>);
