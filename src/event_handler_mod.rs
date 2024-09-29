use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    asset::AssetServer,
    audio::{AudioBundle, PlaybackSettings},
    prelude::{Commands, EventReader, Res},
};

use crate::constants::{CollisionEvent, CollisionSound};

pub struct event_handler_plugin;

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty()
    {
        collision_events.clear();
        // This prevents events staying active on the next frame.
        commands.spawn(AudioBundle {
            source: sound.clone(),
            // auto-despawn the entity when playback finishes
            settings: PlaybackSettings::DESPAWN,
        });
        println!("playing sound...");
    }
}

impl Plugin for event_handler_plugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.add_systems(FixedUpdate, play_collision_sound);
    }
}
