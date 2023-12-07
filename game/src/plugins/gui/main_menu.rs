// Crates
use crate::{plugins::gui::*, AppState};
use bevy::prelude::*;
use game_gui_components::*;

// Bevy Plugin
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        // When entering the state spawn in the main menu
        app.add_state::<MenusState>();
        app.add_systems(OnEnter(AppState::Menus), setup);
        app.add_systems(
            OnExit(AppState::Menus),
            (cleanup, credits::cleanup, settings::cleanup),
        );
        app.add_systems(
            Update,
            (
                play_button_behavior,
                settings_button_behavior,
                credits_button_behavior,
                exit_button_behavior,
            ),
        );
        app.add_systems(OnEnter(MenusState::Credits), credits::setup);
        app.add_systems(OnEnter(MenusState::Settings), settings::setup);
        app.add_systems(OnExit(MenusState::Credits), credits::cleanup);
        app.add_systems(OnExit(MenusState::Settings), settings::cleanup);
    }
}

const TITLE_SPRITE: &str = "title.png";

macro_rules! button_text {
    ($text:expr) => {
        TextBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            text: Text::from_section(
                $text,
                TextStyle {
                    font_size: 64.,
                    ..default()
                },
            ),
            ..default()
        }
    };
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenusState {
    #[default]
    Default,
    Credits,
    Settings,
}

#[derive(Component)]
struct MainMenuRoot;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainMenuRoot, Name::new("Camera")));
    commands
        .spawn((
            NodeBundle {
                background_color: Color::rgb(0.235, 0.039, 0.).into(),
                style: Style {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    padding: UiRect::all(Val::VMin(6.)),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MainMenuRoot,
            Name::new("MainMenu"),
        ))
        .with_children(|parent| {
            // Title Image
            parent.spawn((
                ImageBundle {
                    image: asset_server.load(TITLE_SPRITE).into(),
                    background_color: Color::rgb(1., 0.521, 0.227).into(),
                    style: Style {
                        width: Val::VMin(80.),
                        height: Val::VMin(80.0 / 3.4),
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Title"),
            ));
            // Buttons
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::VMin(80.),
                            height: Val::Percent(60.),
                            margin: UiRect::all(Val::Auto),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("Buttons"),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            game_buttons::new_button_bundle(),
                            PlayButton,
                            Name::new("PlayButton"),
                        ))
                        .with_children(|parent| {
                            parent.spawn(button_text!("Play"));
                        });
                    parent
                        .spawn((
                            game_buttons::new_button_bundle(),
                            SettingsButton,
                            Name::new("SettingsButton"),
                        ))
                        .with_children(|parent| {
                            parent.spawn(button_text!("Settings"));
                        });
                    parent
                        .spawn((
                            game_buttons::new_button_bundle(),
                            CreditsButton,
                            Name::new("CreditsButton"),
                        ))
                        .with_children(|parent| {
                            parent.spawn(button_text!("Credits"));
                        });
                    parent
                        .spawn((
                            game_buttons::new_button_bundle(),
                            ExitButton,
                            Name::new("ExitButton"),
                        ))
                        .with_children(|parent| {
                            parent.spawn(button_text!("Exit"));
                        });
                });
        });
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<MainMenuRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Play button component and a system to go with it
#[derive(Component)]
struct PlayButton;

fn play_button_behavior(
    // Changed<Interaction> makes it only query what has had an Interaction changed in the last frame
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::Game)
        }
    }
}

// Settings button component and a system to go with it
#[derive(Component)]
struct SettingsButton;

fn settings_button_behavior(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    mut next_menus_state: ResMut<NextState<MenusState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_menus_state.set(MenusState::Settings)
        }
    }
}

// Credits button component and a system to go with it
#[derive(Component)]
struct CreditsButton;

fn credits_button_behavior(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CreditsButton>)>,
    mut next_menus_state: ResMut<NextState<MenusState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_menus_state.set(MenusState::Credits)
        }
    }
}

// Exit button component and a system to go with it
#[derive(Component)]
struct ExitButton;

fn exit_button_behavior(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut app_exit_event: ResMut<Events<bevy::app::AppExit>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            app_exit_event.send(bevy::app::AppExit)
        }
    }
}
