use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    AppState,
    Score,
};

use crate::ingame::{
    GRID_SIZE,
    BAR_SIZE,
    Cue,
    Bar,
    TimingEvent,
    ReversalEvent,
    TimingSound,
    ReversalSound,
    TimingEffect,
    ReversalEffect,
    ScoreboardUi,
    GameTimer,
};

const CUE_SPEED: f32 = 7.0;

pub fn cue_movement(
    mut cue_query: Query<(&mut Transform, &mut Cue), With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut reversal_events: EventWriter<ReversalEvent>,
) {
    let Ok((mut cue_transform, mut cue_prop)) = cue_query.get_single_mut() else { return; };
    let cue_x = cue_transform.translation.x;
    let bar_transform = bar_query.single();
    let bar_x = bar_transform.translation.x;

    if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
        reversal_events.send_default();
        // reversal movement
        cue_prop.toggle_move = !cue_prop.toggle_move;
    }

    cue_transform.translation.x += if cue_prop.toggle_move { CUE_SPEED } else { -CUE_SPEED };
}

pub fn spawn_reversal_effect(
    mut reversal_events: EventReader<ReversalEvent>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), With<ReversalEffect>>,
    cue_query: Query<&Cue, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<ReversalEffect>)>,
) {
    if reversal_events.is_empty() { return }
    reversal_events.clear();

    let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let cue_prop = cue_query.single();
    let bar_transform = bar_query.single();
    let bar_xy = bar_transform.translation.xy();

    let effect_transform_x = match cue_prop.toggle_move {
        true => bar_xy.x - BAR_SIZE.x / 2.0,
        false => bar_xy.x + BAR_SIZE.x / 2.0,
    };
    let effect_rotation_z = if cue_prop.toggle_move { 1.5 } else { -1.5 };

    effect_transform.translation = Vec3::new(effect_transform_x, bar_xy.y, 0.0);
    effect_transform.rotation = Quat::from_rotation_z(effect_rotation_z);
    spawner.reset();
}

pub fn play_reversal_sound(
    mut commands: Commands,
    mut reversal_events: EventReader<ReversalEvent>,
    sound: Res<ReversalSound>,
) {
    if reversal_events.is_empty() { return }
    reversal_events.clear();

    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn decide_timing(
    mouse_event: Res<ButtonInput<MouseButton>>,
    cue_query: Query<&Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut timing_events: EventWriter<TimingEvent>,
    mut score: ResMut<Score>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return }
    timing_events.send_default();

    let bar_x = bar_query.single().translation.x;
    let cue_x = cue_query.single().translation.x;

    if cue_x < bar_x + GRID_SIZE as f32 && cue_x > bar_x - GRID_SIZE as f32 {
        **score += 3;
    }
    else if cue_x < bar_x + (GRID_SIZE * 2) as f32 && cue_x > bar_x - (GRID_SIZE * 2) as f32 {
        **score += 2;
    }
    else {
        if **score > 0 { **score -= 1 };
    }
}

pub fn spawn_timing_effect(
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<TimingEffect>, Without<Cue>, Without<Bar>)>,
    mut timing_events: EventReader<TimingEvent>,
    cue_query: Query<&Transform, With<Cue>>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let cue_transform = cue_query.single();

    effect_transform.translation = cue_transform.translation;
    spawner.reset();
}

pub fn play_timing_sound(
    mut commands: Commands,
    mut timing_events: EventReader<TimingEvent>,
    sound: Res<TimingSound>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn scoreboard(
    score: Res<Score>,
    timer: ResMut<GameTimer>,
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

pub fn gametimer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut cue_query: Query<&mut Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Reset timer
        timer.0.reset();
        // Reset cue position
        let bar_transform = bar_query.single();

        for mut cue_transform in &mut cue_query {
            cue_transform.translation.x = bar_transform.translation.x + BAR_SIZE.x / 2.0;
        }
        // Move app state
        app_state.set(AppState::Gameover);
    }
}
