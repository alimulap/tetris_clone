use bevy::{prelude::*, app::{PluginGroupBuilder, AppExit}, window::WindowTheme};

pub fn bevy_default_set() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tetris Clone @alimulap".into(),
            resolution: (400., 600.).into(),
            // fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    })
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn update(
    buttons: Query<&Interaction>,
    mut window: Query<&mut Window>,
) {
    let mut window = window.single_mut();
    for buttons in buttons.iter() {
        if buttons.eq(&Interaction::Hovered) {
            window.cursor.icon = CursorIcon::Pointer;
        } else {
            window.cursor.icon = CursorIcon::Default;
        }
    }
}

pub fn close_on_q(
    // mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::KeyQ) {
            // commands.entity(window).despawn();
            exit.send(AppExit);
        }
    }
}
