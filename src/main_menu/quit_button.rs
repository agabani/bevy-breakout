use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct QuitButton;

#[allow(clippy::needless_pass_by_value)]
pub fn interaction(
    mut event_writer: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            event_writer.send(AppExit);
        }
    }
}
