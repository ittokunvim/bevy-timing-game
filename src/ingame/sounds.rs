use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    PATH_SOUND_GOOD,
    PATH_SOUND_OK,
    PATH_SOUND_PERFECT,
    PATH_SOUND_REVERSAL,
    PATH_SOUND_TIMING,
    AppState,
};

use crate::ingame::{
    GoodEvent,
    OkEvent,
    PerfectEvent,
    ReversalEvent,
    TimingEvent,
};

#[derive(Resource, Deref)]
struct GoodSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct OkSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct PerfectSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct TimingSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct ReversalSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    let good_sound = asset_server.load(PATH_SOUND_GOOD);
    let ok_sound = asset_server.load(PATH_SOUND_OK);
    let perfect_sound = asset_server.load(PATH_SOUND_PERFECT);
    let timing_sound = asset_server.load(PATH_SOUND_TIMING);
    let reversal_sound = asset_server.load(PATH_SOUND_REVERSAL);

    commands.insert_resource(TimingSound(good_sound));
    commands.insert_resource(TimingSound(ok_sound));
    commands.insert_resource(TimingSound(perfect_sound));
    commands.insert_resource(TimingSound(timing_sound));
    commands.insert_resource(ReversalSound(reversal_sound));
}

fn play_good_sound(
    mut good_events: EventReader<GoodEvent>,
    mut commands: Commands,
    sound: Res<GoodSound>,
) {
    if good_events.is_empty() { return }
    good_events.clear();
    // play good sound
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_ok_sound(
    mut ok_events: EventReader<OkEvent>,
    mut commands: Commands,
    sound: Res<OkSound>,
) {
    if ok_events.is_empty() { return }
    ok_events.clear();
    // play ok sound
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_perfect_sound(
    mut perfect_event: EventReader<PerfectEvent>,
    mut commands: Commands,
    sound: Res<PerfectSound>,
) {
    if perfect_event.is_empty() { return }
    perfect_event.clear();
    // play perfect sound
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_timing_sound(
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

fn play_reversal_sound(
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

pub struct SoundsPlugin;

impl Plugin for SoundsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                play_good_sound,
                play_ok_sound,
                play_perfect_sound,
                play_timing_sound,
                play_reversal_sound,
            ).run_if(in_state(AppState::Ingame)));
    }
}
