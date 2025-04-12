use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    asset::AssetServer,
    audio::{AudioBundle, PlaybackSettings},
    prelude::{in_state, Commands, EventReader, IntoSystemConfigs, OnEnter, Query, Res, With},
};

use crate::{
    constants::{CollisionEvent, CollisionSound},
    GameState,
};

use super::{LevelText, Score};

pub struct EventHandlerPlugin;

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred.
    if !collision_events.is_empty() {
        collision_events.clear();
        // This prevents events staying active on the next frame.
        // commands.spawn(AudioBundle {
        //     source: sound.clone(),
        //     // auto-despawn the entity when playback finishes
        //     settings: PlaybackSettings::DESPAWN,
        // });
        println!("playing sound...");
    }
}

impl Plugin for EventHandlerPlugin {
    fn build(&self, app: &mut App) {
        println!("This is the build process now");
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(
                Update,
                play_collision_sound.run_if(in_state(GameState::Game)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CollisionSound(asset_server.load("sounds/hitHurt.wav")));
}
