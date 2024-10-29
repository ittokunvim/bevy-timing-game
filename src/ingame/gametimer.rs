use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GameTimer;

fn update(
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

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)));
    }
}
