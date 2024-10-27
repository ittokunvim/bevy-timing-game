use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_hanabi::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
    Score,
};

use crate::ingame::{
    GRID_SIZE,
    BAR_SIZE,
    TIMINGBTN_SIZE,
    Cue,
    Bar,
    TimingButton,
    TimingEvent,
    ReversalEvent,
    TimingSound,
    ReversalSound,
    TimingEffect,
    ReversalEffect,
    ScoreboardUi,
    AnimationTimer,
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

pub fn spawn_reversal_effect(
    mut reversal_events: EventReader<ReversalEvent>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<ReversalEffect>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<ReversalEffect>)>,
    cue_query: Query<&Cue, With<Cue>>,
) {
    if reversal_events.is_empty() { return }
    reversal_events.clear();

    let Ok((mut effect_spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let bar_transform = bar_query.single();
    let bar_xy = bar_transform.translation.xy();
    let cue_prop = cue_query.single();

    let effect_transform_x = match cue_prop.toggle_move {
        true => bar_xy.x - BAR_SIZE.x / 2.0,
        false => bar_xy.x + BAR_SIZE.x / 2.0,
    };
    let effect_rotation_z = if cue_prop.toggle_move { 1.5 } else { -1.5 };
    // spawn reversal effect
    effect_transform.translation = Vec3::new(effect_transform_x, bar_xy.y, 0.0);
    effect_transform.rotation = Quat::from_rotation_z(effect_rotation_z);
    effect_spawner.reset();
}

pub fn play_reversal_sound(
    mut reversal_events: EventReader<ReversalEvent>,
    mut commands: Commands,
    sound: Res<ReversalSound>,
) {
    if reversal_events.is_empty() { return }
    reversal_events.clear();
    // play reversal sound
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn decide_timing(
    mouse_event: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Transform, &mut TimingButton, &mut TextureAtlas), With<TimingButton>>,
    mut timing_events: EventWriter<TimingEvent>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let Ok((transform, mut prop, mut atlas)) = query.get_single_mut() else { return; };
    let timingbtn_pos = transform.translation.truncate();
    cursor_pos = Vec2::new(cursor_pos.x, -cursor_pos.y + WINDOW_SIZE.y);

    let distance = cursor_pos.distance(timingbtn_pos);

    if distance < TIMINGBTN_SIZE as f32 - 10.0 {
        timing_events.send_default();
        // animation timingbtn
        prop.pushed = true;
        atlas.index = prop.last;
     }
}

pub fn animation_timingbtn(
    time: Res<Time>,
    mut query: Query<(&mut TimingButton, &mut AnimationTimer, &mut TextureAtlas), With<TimingButton>>,
) {
    let Ok((mut prop, mut timer, mut atlas)) = query.get_single_mut() else { return; };

    if !prop.pushed { return; }
    timer.tick(time.delta());
    if timer.just_finished() {
        prop.pushed = false;
        atlas.index = prop.first;
    }
}

pub fn score_point(
    mut timing_events: EventReader<TimingEvent>,
    cue_query: Query<&Transform, (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut score: ResMut<Score>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let cue_x = cue_query.single().translation.x;
    let bar_x = bar_query.single().translation.x;

    // if cue was just timing
    if cue_x < bar_x + GRID_SIZE as f32 && cue_x > bar_x - GRID_SIZE as f32 {
        **score += 3;
    }
    // if cue was good timing
    else if cue_x < bar_x + (GRID_SIZE * 2) as f32 && cue_x > bar_x - (GRID_SIZE * 2) as f32 {
        **score += 2;
    }
    else {
        if **score > 0 { **score -= 1 };
    }
}

pub fn spawn_timing_effect(
    mut timing_events: EventReader<TimingEvent>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<TimingEffect>, Without<Cue>)>,
    cue_query: Query<&Transform, With<Cue>>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let cue_transform = cue_query.single();
    // spawn timing effect
    effect_transform.translation = cue_transform.translation;
    spawner.reset();
}

pub fn play_timing_sound(
    mut timing_events: EventReader<TimingEvent>,
    mut commands: Commands,
    sound: Res<TimingSound>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();
    // play timing sound
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

pub fn scoreboard(
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
    score: Res<Score>,
    timer: ResMut<GameTimer>,
) {
    let mut text = scoreboard_query.single_mut();
    // write score and timer
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
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
