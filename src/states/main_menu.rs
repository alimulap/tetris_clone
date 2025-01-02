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
                Text(String::from("MAIN MENU")),
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
                    StartGameButton,
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
                    BorderRadius::all(Val::Px(5.0)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text(String::from("START GAME")),
                        TextFont {
                            font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                            font_size: 19.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
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
