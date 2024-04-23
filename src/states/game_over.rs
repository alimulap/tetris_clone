#![allow(unused)]

use bevy::prelude::*;

use super::AppState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::GameOver), setup)
            .add_systems(
                Update,
                (on_click_restart, input_handler).run_if(in_state(AppState::GameOver)),
            )
            .add_systems(OnExit(AppState::GameOver), cleanup);
    }
}

#[derive(Component)]
struct GameOver;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOver,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "GAME OVER",
                    TextStyle {
                        font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    margin: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                }),
            );
            parent
                .spawn((
                    RestartButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(175.0),
                            height: Val::Px(35.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(1.0)),
                            ..Default::default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        border_color: Color::rgb(0.8, 0.8, 0.8).into(),
                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "RESTART GAME",
                        TextStyle {
                            font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                            font_size: 23.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

#[derive(Component)]
struct RestartButton;

fn on_click_restart(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    restart_button: Query<&Interaction, With<RestartButton>>,
) {
    if restart_button.single().eq(&Interaction::Pressed) {
        next_state.set(AppState::LoadGame);
    }
}

fn input_handler(
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::LoadGame);
    }
}

fn cleanup(mut commands: Commands, gameover: Query<Entity, With<GameOver>>) {
    commands.entity(gameover.single()).despawn_recursive();
}
