// Crates
use bevy::prelude::*;
use bevy_inspector_egui::quick;
use game::{plugins::*, AppState};
use game_gui_components::{game_buttons::GameButtonsPlugin, *};

/// 'main' is intended only to initialize the app, the plugins handle the rest
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins((
            DefaultPlugins,
            //quick::WorldInspectorPlugin::default(),
            game_buttons::GameButtonsPlugin,
            gui::main_menu::MainMenuPlugin,
        ))
        .run();
}

/// *App-wide* setup goes here
fn setup() {
    println!("Game loaded! :D")
}
