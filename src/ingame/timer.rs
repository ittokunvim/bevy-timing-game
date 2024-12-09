use bevy::prelude::*;

use crate::{
    AppState,
    Score,
};
use crate::ingame::{
    SCORE_CLEARPOINT,
    GameTimer,
};

fn update(
    mut timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    score: Res<Score>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("timer: reset");
        timer.0.reset();
        if **score < SCORE_CLEARPOINT {
            println!("timer: moved state to Gameover from Ingame");
            next_state.set(AppState::Gameover);
        } else {
            println!("timer: moved state to Gameclear from Ingame");
            next_state.set(AppState::Gameclear);
        }
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
