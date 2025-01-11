use bevy::prelude::*;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonPressed>()
            .add_systems(Update, button_press_handler);
    }
}

pub fn add<M>(
    name: String,
    text: String,
    on_click: impl IntoSystem<(), (), M> + Send + 'static,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) -> Entity {
    // let on_click = cmd.register_system(on_click);
    let name = Name::new(name.clone());
    let id = parent
        .spawn((
            name.clone(),
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
                Text(text),
                TextFont {
                    font: asset_server.load("fonts/Montserrat-Regular.ttf"),
                    font_size: 19.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        })
        .id();

    parent.enqueue_command(move |world: &mut World| {
        let on_click = world.register_system(on_click);
        let mut observer = Observer::new(
            move |trigger: Trigger<ButtonPressed>,
                  mut cmd: Commands,
                  button: Query<&Name, With<Button>>| {
                if button.get(trigger.entity()).unwrap().eq(&name) {
                    cmd.run_system(on_click);
                };
            },
        );
        observer.watch_entity(id);
        world.spawn(observer);
    });

    id
}

// pub fn add(name: &str, on_click: Observer, mut commands: Commands, asset_server: Res<AssetServer>) {
// }

#[derive(Event)]
struct ButtonPressed;

fn button_press_handler(
    mut cmd: Commands,
    button_query: Query<(Entity, &Interaction), With<Button>>,
) {
    for (entity, interaction) in button_query.iter() {
        if Interaction::Pressed.eq(interaction) {
            cmd.trigger_targets(ButtonPressed, entity);
        }
    }
}
