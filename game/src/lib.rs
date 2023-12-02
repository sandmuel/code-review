// Crates
use bevy::prelude::*;

// Modules
pub mod plugins;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menus,
    Game,
}
