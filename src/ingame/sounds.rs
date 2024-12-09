use bevy::prelude::*;

use crate::{
    PATH_SOUND_GOOD,
    PATH_SOUND_OK,
    PATH_SOUND_PERFECT,
    PATH_SOUND_REVERSAL,
    AppState,
    Config,
};

use crate::ingame::{
    GoodEvent,
    OkEvent,
    PerfectEvent,
    ReversalEvent,
};

#[derive(Resource, Deref)]
struct PerfectSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct GoodSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct OkSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct ReversalSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("sounds: setup");
    let perfect_sound = asset_server.load(PATH_SOUND_PERFECT);
    commands.insert_resource(PerfectSound(perfect_sound));

    let good_sound = asset_server.load(PATH_SOUND_GOOD);
    commands.insert_resource(GoodSound(good_sound));

    let ok_sound = asset_server.load(PATH_SOUND_OK);
    commands.insert_resource(OkSound(ok_sound));

    let reversal_sound = asset_server.load(PATH_SOUND_REVERSAL);
    commands.insert_resource(ReversalSound(reversal_sound));
}

fn play_perfect_sound(
    mut events: EventReader<PerfectEvent>,
    mut commands: Commands,
    sound: Res<PerfectSound>,
) {
    if events.is_empty() { return }
    events.clear();
    println!("sounds: perfect");
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_good_sound(
    mut events: EventReader<GoodEvent>,
    mut commands: Commands,
    sound: Res<GoodSound>,
) {
    if events.is_empty() { return }
    events.clear();
    println!("sounds: good");
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_ok_sound(
    mut events: EventReader<OkEvent>,
    mut commands: Commands,
    sound: Res<OkSound>,
) {
    if events.is_empty() { return }
    events.clear();
    println!("sounds: ok");
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn play_reversal_sound(
    mut events: EventReader<ReversalEvent>,
    mut commands: Commands,
    sound: Res<ReversalSound>,
) {
    if events.is_empty() { return }
    events.clear();
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
                play_reversal_sound,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}
