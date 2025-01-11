use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorState::Default)
            .add_systems(Update, button_hover);
    }
}

#[derive(Resource)]
pub enum CursorState {
    Default,
    Pointer,
}

pub fn button_hover(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    buttons: Query<&Interaction>,
    mut cursor_state: ResMut<CursorState>,
) {
    let mut hover = false;
    for buttons in buttons.iter() {
        if buttons.eq(&Interaction::Hovered) {
            hover = true;
        }
    }

    if hover {
        commands
            .entity(*window)
            .insert(CursorIcon::System(SystemCursorIcon::Pointer));
        *cursor_state = CursorState::Pointer;
    } else {
        commands
            .entity(*window)
            .insert(CursorIcon::System(SystemCursorIcon::Default));
        *cursor_state = CursorState::Default;
    }
}
