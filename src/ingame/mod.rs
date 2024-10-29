use bevy::prelude::*;

mod bar;
mod camera;
mod character;
mod cue;
// mod effects;
mod gametimer;
mod ldtk;
mod pausebutton;
mod scoreboard;
mod sounds;
mod timingbutton;

const GAMETIME_LIMIT: f32 = 10.0;

#[derive(Event, Default)]
struct PerfectEvent;

#[derive(Event, Default)]
struct GoodEvent;

#[derive(Event, Default)]
struct OkEvent;

#[derive(Event, Default)]
struct BadEvent;

#[derive(Event, Default)]
struct TimingEvent;

#[derive(Event, Default)]
struct ReversalEvent;

#[derive(Resource)]
struct GameTimer(Timer);

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PerfectEvent>()
            .add_event::<GoodEvent>()
            .add_event::<OkEvent>()
            .add_event::<BadEvent>()
            .add_event::<TimingEvent>()
            .add_event::<ReversalEvent>()
            .insert_resource(GameTimer(Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)))
            .add_plugins(bar::BarPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(character::CharacterPlugin)
            .add_plugins(cue::CuePlugin)
            // .add_plugins(effects::EffectsPlugin)
            .add_plugins(gametimer::GameTimerPlugin)
            .add_plugins(ldtk::LdtkPlugin)
            .add_plugins(pausebutton::PauseButtonPlugin)
            .add_plugins(scoreboard::ScoreboardPlugin)
            .add_plugins(sounds::SoundsPlugin)
            .add_plugins(timingbutton::TimingButtonPlugin);
    }
}
