// Crates
use bevy::prelude::*;

#[derive(Component)]
pub struct CreditsRoot;

pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Local(1),
                background_color: Color::rgba(0., 0., 0., 0.6).into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    ..default()
                },
                ..default()
            },
            CreditsRoot,
            Name::new("Credits"),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    background_color: Color::rgba(0.08, 0.08, 0.08, 0.99).into(),
                    style: Style {
                        width: Val::VMin(75.),
                        height: Val::Vh(90.),
                        margin: UiRect::all(Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            margin: UiRect {
                                left: Val::Auto,
                                right: Val::Auto,
                                top: Val::Vh(3.0),
                                bottom: Val::Vh(3.0),
                            },
                            ..default()
                        },
                        text: Text::from_section(
                            "Credits",
                            TextStyle {
                                font_size: 64.,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<CreditsRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
