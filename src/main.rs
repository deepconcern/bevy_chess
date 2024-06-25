mod app_state;
mod game;
mod main_menu;

use app_state::AppState;
use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(GamePlugin);
    app.add_plugins(MainMenuPlugin);
    app.insert_resource(ClearColor(Color::rgb(
        115.0 / 255.0,
        205.0 / 255.0,
        75.0 / 255.0,
    )));
    app.add_systems(Startup, setup);
    app.init_state::<AppState>();
    
    app.run();
}
