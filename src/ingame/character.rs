use bevy::prelude::*;

use crate::{
    PATH_IMAGE_CHARACTER,
    AppState,
    Config,
};
use crate::ingame::{
    GRID_SIZE,
    PerfectEvent,
    GoodEvent,
    OkEvent,
    BadEvent,
};

const IMAGE_SIZE: u32 = 32;
const SIZE: f32 = 64.0;
const IMAGE_COLUMN: u32 = 4;
const IMAGE_ROW: u32 = 5;
const IDLE_RANGE: (usize, usize) = (0, 3);
const IDLE_SECS: f32 = 0.2;
const PERFECT_RANGE: (usize, usize) = (4, 6);
const PERFECT_SECS: f32 = 0.3;
const GOOD_RANGE: (usize, usize) = (8, 11);
const GOOD_SECS: f32 = 0.2;
const OK_RANGE: (usize, usize) = (12, 14);
const OK_SECS: f32 = 0.3;
const BAD_RANGE: (usize, usize) = (16, 18);
const BAD_SECS: f32 = 0.3;

#[derive(Component)]
struct Character {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    config: Res<Config>,
) {
    if !config.setup_ingame { return }

    println!("character: setup");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(IMAGE_SIZE),
        IMAGE_COLUMN,
        IMAGE_ROW,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = Character { first: IDLE_RANGE.0, last: IDLE_RANGE.1 };
    let (x, y, z) = (
        0.0,
        GRID_SIZE * 6.0,
        0.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_CHARACTER),
            transform: Transform::from_xyz(x, y, z),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(IDLE_SECS, TimerMode::Repeating))
    ))
    .insert(Name::new("character"));
}

fn update(
    mut query: Query<(&mut Character, &mut AnimationTimer, &mut TextureAtlas)>,
    mut perfect_events: EventReader<PerfectEvent>,
    mut good_events: EventReader<GoodEvent>,
    mut ok_events: EventReader<OkEvent>,
    mut bad_events: EventReader<BadEvent>,
    time: Res<Time>,
) {
    let Ok((mut prop, mut timer, mut atlas)) = query.get_single_mut() else { return };
    let atlas_index = atlas.index;
    let mut closure = |range, secs| {
        (prop.first, prop.last) = range;
        atlas.index = range.0;
        timer.0 = Timer::from_seconds(secs, TimerMode::Repeating);
    };

    if !perfect_events.is_empty() {
        println!("character: perfect");
        perfect_events.clear();
        closure(PERFECT_RANGE, PERFECT_SECS);
    }
    else if !good_events.is_empty() {
        println!("character: good");
        good_events.clear();
        closure(GOOD_RANGE, GOOD_SECS);
    }
    else if !ok_events.is_empty() {
        println!("character: ok");
        ok_events.clear();
        closure(OK_RANGE, OK_SECS);
    }
    else if !bad_events.is_empty() {
        println!("character: bad");
        bad_events.clear();
        closure(BAD_RANGE, BAD_SECS);
    }

    if [PERFECT_RANGE.1, GOOD_RANGE.1, OK_RANGE.1, BAD_RANGE.1,].contains(&atlas_index) {
        closure(IDLE_RANGE, IDLE_SECS);
    }

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = if atlas.index == prop.last { prop.first } else { atlas.index + 1 }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Character>>,
) {
    println!("character: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(OnEnter(AppState::Mainmenu), despawn)
        ;
    }
}
