use bevy::prelude::*;

use crate::AppState;
use crate::ingame::GameTimer;

fn update(
    mut timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("timer: reset");
        timer.0.reset();
        println!("timer: moved state to Gameover from Ingame");
        next_state.set(AppState::Gameover);
    }
}

pub struct GameTimerPlugin;

impl Plugin for GameTimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
        ;
    }
}
