use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
pub struct QuitButton {}

#[allow(clippy::needless_pass_by_value)]
pub fn interaction(
    mut event_writer: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    if let Ok(interaction) = query.get_single() {
        match interaction {
            Interaction::Hovered | Interaction::None => {
                // TODO: change color
            }
            Interaction::Pressed => event_writer.send(AppExit),
        }
    };
}
