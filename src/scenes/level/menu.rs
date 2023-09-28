use bevy::prelude::*;

use crate::scenes::SceneState;

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn escape(mut next_state: ResMut<NextState<SceneState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        next_state.set(SceneState::MainMenu);
    }
}
