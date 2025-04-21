mod eneymy_mod;
mod event_handler_mod;
mod player_jet_mod;
mod space_point_plugin_mod;

use crate::GameState;
use bevy::prelude::*;
use eneymy_mod::EnemyPlugin;
use event_handler_mod::EventHandlerPlugin;
use player_jet_mod::{GameEntity, JetPlugin};

pub const MY_ORANGE: Color = Color::srgb(222.0 / 255.0, 112.0 / 255.0, 40.0 / 255.0);

#[derive(Component)]
pub struct LevelText;

#[derive(Resource)]
pub struct Score(usize);

impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

pub fn game_plugin(app: &mut App) {
    app.init_resource::<Score>()
        .add_systems(Startup, setup_text)
        .add_plugins((JetPlugin, EnemyPlugin, EventHandlerPlugin))
        .add_systems(OnExit(GameState::Game), despawn_game);
}

fn despawn_game(mut commands: Commands, mut query: Query<Entity, With<GameEntity>>) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands
        .spawn((Text::new(format!("Score : ")), LevelText))
        .with_child(TextSpan::new("0"));
}
