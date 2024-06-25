mod main_menu;

use bevy::{prelude::*, render::camera::RenderTarget, window::PrimaryWindow};
use main_menu::{cleanup_main_menu, setup_main_menu};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum AppState {
    Game,
    #[default]
    MainMenu,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    Black,
    GameOver,
    #[default]
    White,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::rgb(
            115.0 / 255.0,
            205.0 / 255.0,
            75.0 / 255.0,
        )))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .run();
}
