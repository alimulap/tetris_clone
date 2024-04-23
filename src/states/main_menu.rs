use bevy::prelude::*;

use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(
                Update,
                (on_click_start_game, input_handler).run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup);
    }
}

#[derive(Component)]
struct MainMenu;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            MainMenu,
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
                    "MAIN MENU",
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
                    StartGameButton,
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(155.0),
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
                        "START GAME",
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
struct StartGameButton;

fn on_click_start_game(
    mut next_state: ResMut<NextState<AppState>>,
    start_button: Query<&Interaction, With<StartGameButton>>,
) {
    if start_button.single().eq(&Interaction::Pressed) {
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

fn cleanup(mut commands: Commands, main_menu: Query<Entity, With<MainMenu>>) {
    commands.entity(main_menu.single()).despawn_recursive();
}
