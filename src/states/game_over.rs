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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(String::from("GAME OVER")),
                TextFont {
                    font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                    font_size: 40.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    margin: UiRect::all(Val::Px(15.0)),
                    ..Default::default()
                },
            ));
            parent
                .spawn((
                    RestartButton,
                    Button,
                    Node {
                        width: Val::Px(155.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                    BorderColor(Color::srgb(0.8, 0.8, 0.8)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text(String::from("RESTART GAME")),
                        TextFont {
                            font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                            font_size: 23.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
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
