use bevy::{color::palettes::css::RED, prelude::*};

use super::{super::GameState, despawn_screen};

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), setup_splash_screen)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");
    commands
        .spawn((
            (Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(10.0)),

                //background_color : BackgroundColor(Color::srgb(0.8, 0.4, 0.2)),
                // border_color: BorderColor(Color::srgb(0.8, 0.4, 0.2)),
                //border_radius : BorderRadius::MAX,
                ..default()
            }),
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: icon,
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Game);
    }
}
