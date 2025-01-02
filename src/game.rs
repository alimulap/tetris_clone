use bevy::{
    app::{AppExit, PluginGroupBuilder},
    prelude::*,
    window::{SystemCursorIcon, WindowTheme},
    winit::cursor::CursorIcon,
};

pub fn bevy_default_set() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // title: "Tetris Clone @alimulap".into(),
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
    commands.spawn(Camera2d);
}

pub fn update(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    buttons: Query<&Interaction>,
) {
    for buttons in buttons.iter() {
        if buttons.eq(&Interaction::Hovered) {
            commands
                .entity(*window)
                .insert(CursorIcon::System(SystemCursorIcon::Pointer));
        } else {
            commands
                .entity(*window)
                .insert(CursorIcon::System(SystemCursorIcon::Default));
        }
    }
}

pub fn close_on_q(
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::KeyQ) {
            exit.send(AppExit::Success);
        }
    }
}
