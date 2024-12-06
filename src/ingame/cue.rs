use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::AppState;
use crate::ingame::{
    PerfectEvent,
    GoodEvent,
    OkEvent,
    BadEvent,
    TimingEvent,
    ReversalEvent,
    GameTimer,
};
use crate::ingame::bar::{
    GRID_SIZE,
    SIZE as BAR_SIZE,
    Bar,
};

#[derive(Default, Component)]
pub struct Cue {
    pub toggle_move: bool,
}

#[derive(Default, Bundle, LdtkEntity)]
struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

const CUE_SPEED: f32 = 7.0;

fn movement(
    mut cue_query: Query<(&mut Transform, &mut Cue), (With<Cue>, Without<Bar>)>,
    mut reversal_events: EventWriter<ReversalEvent>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
) {
    let Ok((mut cue_transform, mut cue_prop)) = cue_query.get_single_mut() else { return; };
    let bar_transform = bar_query.single();
    let cue_x = cue_transform.translation.x;
    let bar_x = bar_transform.translation.x;

    if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
        println!("cue: reversal");
        reversal_events.send_default();
        cue_prop.toggle_move = !cue_prop.toggle_move;
    }
    // move cue
    cue_transform.translation.x +=
        if cue_prop.toggle_move { CUE_SPEED } else { -CUE_SPEED }
}

fn send_events(
    mut timing_events: EventReader<TimingEvent>,
    mut perfect_events: EventWriter<PerfectEvent>,
    mut good_events: EventWriter<GoodEvent>,
    mut ok_events: EventWriter<OkEvent>,
    mut bad_events: EventWriter<BadEvent>,
    cue_query: Query<&Transform, (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let cue_x = cue_query.single().translation.x;
    let bar_x = bar_query.single().translation.x;

    if cue_x < bar_x + GRID_SIZE && cue_x > bar_x - GRID_SIZE {
        println!("cue: perfect");
        perfect_events.send_default();
    }
    else if cue_x < bar_x + (GRID_SIZE * 2.0) && cue_x > bar_x - (GRID_SIZE * 2.0) {
        println!("cue: good");
        good_events.send_default();
    }
    else if cue_x < bar_x + (GRID_SIZE * 4.0) && cue_x > bar_x - (GRID_SIZE * 4.0) {
        println!("cue: ok");
        ok_events.send_default();
    }
    else {
        println!("cue: bad");
        bad_events.send_default();
    }
}

fn reset_position(
    timer: ResMut<GameTimer>,
    mut cue_query: Query<&mut Transform, (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
) {
    if timer.0.just_finished() {
        let mut cue_transform = cue_query.single_mut();
        let bar_transform = bar_query.single();

        cue_transform.translation.x = bar_transform.translation.x + BAR_SIZE.x / 2.0;
    }
}

pub struct CuePlugin;

impl Plugin for CuePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<CueBundle>("Cue")
            .add_systems(Update, (
                movement,
                send_events,
                reset_position,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}
