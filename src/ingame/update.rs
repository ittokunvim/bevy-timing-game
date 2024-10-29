use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GameTimer;

pub fn gametimer(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        timer.0.reset();
        // Move app state
        app_state.set(AppState::Gameover);
    }
}
