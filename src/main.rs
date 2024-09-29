mod space_point_plugin_mod;
mod player_jet_mod;
mod constants;
mod eneymy_mod;
mod utils;
mod event_handler_mod;

use constants::CollisionSound;
use event_handler_mod::event_handler_plugin;
use player_jet_mod::JetPlugin;
use eneymy_mod::EnemyPlugin;

use bevy::{
    core_pipeline::tonemapping::Tonemapping, prelude::*
};


fn setup_camera(
    mut commands: Commands,
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

    commands.insert_resource(CollisionSound(asset_server.load("sounds/hitHurt.wav")))
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_plugins((
            JetPlugin,
            EnemyPlugin,
            event_handler_plugin
        ))
        .run();
}
