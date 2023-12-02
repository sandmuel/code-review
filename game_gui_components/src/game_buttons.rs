// Crates
use bevy::prelude::*;

// Bevy Plugin
pub struct GameButtonsPlugin;

impl Plugin for GameButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

pub fn new_button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            border: UiRect::all(Val::Vh(0.5)),
            margin: UiRect::all(Val::Vh(1.)),
            ..default()
        },
        ..default()
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *background_color = Color::rgb(0.13, 0.1, 0.12).into();
                *border_color = Color::rgb(1., 0.521, 0.227).into();
            }
            Interaction::Hovered => {
                *background_color = Color::rgb(0.33, 0.3, 0.32).into();
                *border_color = Color::rgb(1., 0.521, 0.227).into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.23, 0.2, 0.22).into();
                *border_color = Color::rgba(0., 0., 0., 0.).into();
            }
        }
    }
}
