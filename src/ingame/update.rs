use bevy::prelude::*;

use crate::AppState;
use crate::ingame::{
    BAR_SIZE,
    Cue,
    Bar,
    ReversalEvent,
    GameTimer,
};

const CUE_SPEED: f32 = 7.0;

pub fn cue_movement(
    mut cue_query: Query<(&mut Transform, &mut Cue), (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut reversal_events: EventWriter<ReversalEvent>,
) {
    let Ok((mut cue_transform, mut cue_prop)) = cue_query.get_single_mut() else { return; };
    let cue_x = cue_transform.translation.x;
    let bar_transform = bar_query.single();
    let bar_x = bar_transform.translation.x;

    if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
        // send reversal event
        reversal_events.send_default();
        // reverse cue
        cue_prop.toggle_move = !cue_prop.toggle_move;
    }
    // move cue
    cue_transform.translation.x += if cue_prop.toggle_move { CUE_SPEED } else { -CUE_SPEED };
}

pub fn gametimer(
    mut timer: ResMut<GameTimer>,
    time: Res<Time>,
    mut cue_query: Query<&mut Transform, (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        timer.0.reset();

        let mut cue_transform = cue_query.single_mut();
        let bar_transform = bar_query.single();
        // Reset cue position
        cue_transform.translation.x = bar_transform.translation.x + BAR_SIZE.x / 2.0;
        // Move app state
        app_state.set(AppState::Gameover);
    }
}
