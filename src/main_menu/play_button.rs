use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct PlayButton {}

#[allow(clippy::needless_pass_by_value)]
pub fn interaction(
    mut next_state: ResMut<NextState<GameState>>,
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for &interaction in &query {
        if interaction == Interaction::Pressed {
            next_state.set(GameState::Level);
        }
    }
}
