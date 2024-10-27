use bevy::prelude::*;

use crate::{
    PATH_SOUND_REVERSAL,
    PATH_SOUND_TIMING,
    AppState,
};

use crate::ingame::{
    ReversalEvent,
    TimingEvent,
};

#[derive(Resource, Deref)]
struct TimingSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct ReversalSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let cue_timing_sound = asset_server.load(PATH_SOUND_TIMING);
    let cue_reversal_sound = asset_server.load(PATH_SOUND_REVERSAL);

    commands.insert_resource(TimingSound(cue_timing_sound));
    commands.insert_resource(ReversalSound(cue_reversal_sound));
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
                play_timing_sound,
                play_reversal_sound,
            ).run_if(in_state(AppState::Ingame)));
    }
}
