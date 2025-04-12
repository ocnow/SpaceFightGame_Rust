mod splash_screen;

use super::{GameState, Volume};
use bevy::prelude::*;
use splash_screen::splash_plugin;
//public setting resources

pub fn menu_plugin(app: &mut App) {
    app.insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_plugins(splash_plugin);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
