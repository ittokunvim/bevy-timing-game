use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    AppState,
    Score,
};

use crate::pause::update_pausebtn;

mod setup;

const GRID_SIZE: i32 = 16;
const BAR_SIZE: Vec2 = Vec2::new((GRID_SIZE * 32) as f32, (GRID_SIZE * 2) as f32);
const GAMETIME_LIMIT: f32 = 10.0;
const CUE_SPEED: f32 = 7.0;

#[derive(Default, Component, Debug)]
struct Cue {
    toggle_move: bool,
}

#[derive(Default, Component)]
struct Bar;

#[derive(Default, Bundle, LdtkEntity)]
struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

#[derive(Event, Default)]
struct TimingEvent;

#[derive(Event, Default)]
struct ReversalEvent;

#[derive(Resource, Deref)]
struct TimingSound(Handle<AudioSource>);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct TimingEffect;

#[derive(Component)]
struct ReversalEffect;

#[derive(Resource, Deref)]
struct ReversalSound(Handle<AudioSource>);

#[derive(Resource)]
struct GameTimer(Timer);

fn cue_movement(
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

fn decide_timing(
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

fn play_timing_sound(
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

fn spawn_timing_effect(
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

fn play_reversal_sound(
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

fn spawn_reversal_effect(
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

fn update_scoreboard(
    score: Res<Score>,
    timer: ResMut<GameTimer>,
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

fn update_gametimer(
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

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TimingEvent>()
            .add_event::<ReversalEvent>()
            .insert_resource(GameTimer(Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)))
            .register_ldtk_entity::<CueBundle>("Cue")
            .add_systems(OnEnter(AppState::Ingame), (
                setup::component,
                setup::effect,
            ))
            .add_systems(Update, (
                cue_movement,
                decide_timing,
                play_timing_sound,
                play_reversal_sound,
                spawn_timing_effect,
                spawn_reversal_effect,
                update_scoreboard,
                update_gametimer,
                update_pausebtn,
            ).run_if(in_state(AppState::Ingame)));
    }
}
