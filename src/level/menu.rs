use bevy::prelude::*;

use crate::GameState;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn escape(mut next_state: ResMut<NextState<GameState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}
