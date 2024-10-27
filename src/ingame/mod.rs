use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;

mod setup;
mod update;

mod effects;

const GRID_SIZE: i32 = 16;
const GAMETIME_LIMIT: f32 = 10.0;
const BAR_SIZE: Vec2 = Vec2::new((GRID_SIZE * 32) as f32, (GRID_SIZE * 2) as f32);
const TIMINGBTN_SIZE: u32 = 64;

#[derive(Default, Component, Debug)]
struct Cue {
    toggle_move: bool,
}

#[derive(Default, Component)]
struct Bar;

#[derive(Default, Component)]
struct TimingButton {
    pushed: bool,
    first: usize,
    last: usize,
}

#[derive(Event, Default)]
struct TimingEvent;

#[derive(Event, Default)]
struct ReversalEvent;

#[derive(Resource, Deref)]
struct TimingSound(Handle<AudioSource>);

#[derive(Resource, Deref)]
struct ReversalSound(Handle<AudioSource>);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Resource)]
struct GameTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Default, Bundle, LdtkEntity)]
struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<TimingEvent>()
            .add_event::<ReversalEvent>()
            .insert_resource(GameTimer(Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)))
            .register_ldtk_entity::<CueBundle>("Cue")
            .add_plugins(effects::EffectsPlugin)
            .add_systems(OnEnter(AppState::Ingame), (
                setup::component,
            ))
            .add_systems(Update, (
                update::cue_movement,
                update::play_reversal_sound,
                update::decide_timing,
                update::animation_timingbtn,
                update::score_point,
                update::play_timing_sound,
                update::scoreboard,
                crate::pause::update_pausebtn,
                update::gametimer,
            ).run_if(in_state(AppState::Ingame)));
    }
}
