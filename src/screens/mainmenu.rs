use bevy::prelude::*;

use crate::modules::button;

use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(
                Update,
                (input_handler,).run_if(in_state(AppState::MainMenu)),
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
            button::add(
                "startgame".to_owned(),
                "START GAME".to_owned(),
                |mut next_state: ResMut<NextState<AppState>>| next_state.set(AppState::LoadGame),
                parent,
                &asset_server,
            );
        });
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
