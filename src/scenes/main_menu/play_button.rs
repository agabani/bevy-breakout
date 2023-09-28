use bevy::prelude::*;

use crate::scenes::SceneState;

#[derive(Component)]
#[cfg_attr(feature = "dev", derive(Reflect))]
pub struct PlayButton;

#[allow(clippy::needless_pass_by_value)]
pub fn interaction(
    mut next_state: ResMut<NextState<SceneState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            next_state.set(SceneState::Level);
        }
    }
}
