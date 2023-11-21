use bevy::{prelude::*, app::PluginGroupBuilder, window::WindowTheme};

pub fn bevy_default_set() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tetris Clone @alimulap".into(),
            resolution: (400., 600.).into(),
            fit_canvas_to_parent: true,
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
