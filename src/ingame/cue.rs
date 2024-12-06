use bevy::prelude::*;

use crate::{
    PATH_IMAGE_CUE,
    AppState,
    Config,
};
use crate::ingame::{
    GRID_SIZE,
    PerfectEvent,
    GoodEvent,
    OkEvent,
    BadEvent,
    TimingEvent,
    ReversalEvent,
};
use crate::ingame::bar::{
    SIZE as BAR_SIZE,
    Bar,
};

#[derive(Component)]
pub struct Cue;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

const SIZE: f32 = 48.0;
const SPEED: f32 = 200.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("cue: setup");
    let (x, y, z): (f32, f32, f32) = (
        GRID_SIZE * 10.0,
        GRID_SIZE * 10.0,
        99.0,
    );
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_CUE),
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        },
        Cue,
        Velocity(Vec2::new(-SPEED, 0.0)),
    ));
}

fn apply_velocity(
    mut cue_query: Query<(&mut Transform, &mut Velocity), (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    time_step: Res<Time<Fixed>>,
    mut events: EventWriter<ReversalEvent>,
) {
    let Ok((mut cue_transform, mut cue_velocity)) =
        cue_query.get_single_mut() else { return };
    let bar_transform = bar_query.single();
    let cue_x = cue_transform.translation.x;
    let bar_x = bar_transform.translation.x;

    if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
        events.send_default();
        // reversal velocity
        cue_velocity.x = -cue_velocity.x;
    }
    // update cue x
    cue_transform.translation.x += cue_velocity.x * time_step.delta().as_secs_f32();
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

    // perfect
    if cue_x < bar_x + GRID_SIZE && cue_x > bar_x - GRID_SIZE {
        perfect_events.send_default();
    }
    // good
    else if cue_x < bar_x + (GRID_SIZE * 2.0) && cue_x > bar_x - (GRID_SIZE * 2.0) {
        good_events.send_default();
    }
    // ok
    else if cue_x < bar_x + (GRID_SIZE * 4.0) && cue_x > bar_x - (GRID_SIZE * 4.0) {
        ok_events.send_default();
    }
    // bad
    else {
        println!("cue: bad");
        bad_events.send_default();
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Cue>>,
) {
    println!("cue: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct CuePlugin;

impl Plugin for CuePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                apply_velocity,
                send_events,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnEnter(AppState::Gameover), despawn)
        ;
    }
}
