mod constants;
mod game;
mod menu;
mod utils;

use constants::CollisionSound;
use game::game_plugin;
use menu::menu_plugin;

use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

//public states
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    println!("Starting game...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .init_state::<GameState>()
        .add_plugins((menu_plugin, game_plugin))
        .run();
}
