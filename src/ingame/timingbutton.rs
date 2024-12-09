use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    CURSOR_RANGE,
    PATH_IMAGE_TIMINGBUTTON,
    AppState,
    Config,
};
use crate::ingame::TimingEvent;

const IMAGE_SIZE: u32 = 64;
const IMAGE_COLUMN: u32 = 2;
const IMAGE_ROW: u32 = 1;
const SIZE: f32 = 64.0;

#[derive(Component)]
struct TimingButton {
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

    println!("timingbutton: setup");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(IMAGE_SIZE),
        IMAGE_COLUMN,
        IMAGE_ROW,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = TimingButton { first: 0, last: 1 };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_TIMINGBUTTON),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    ))
    .insert(Name::new("timingbutton"));
}

fn update(
    mut query: Query<(&mut TextureAtlas, &Transform, &TimingButton), With<TimingButton>>,
    mut timing_events: EventWriter<TimingEvent>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let Ok((mut atlas, transform, prop)) = query.get_single_mut() else { return };
    let timingbtn_pos = transform.translation.truncate();
    cursor_pos = Vec2::new(
        cursor_pos.x - window.width() / 2.0, 
        -cursor_pos.y + window.height() / 2.0,
    );

    let distance = cursor_pos.distance(timingbtn_pos);

    if distance < SIZE - CURSOR_RANGE {
        timing_events.send_default();
        println!("timingbutton: push");
        atlas.index = prop.last;
     }
}

fn animation(
    mut query: Query<(&mut TextureAtlas, &mut AnimationTimer, &TimingButton), With<TimingButton>>,
    time: Res<Time>,
) {
    let Ok((mut atlas, mut timer, prop)) = query.get_single_mut() else { return };

    if atlas.index == prop.first { return }

    timer.tick(time.delta());
    if timer.just_finished() {
        atlas.index = prop.first;
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<TimingButton>>,
) {
    println!("timingbutton: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct TimingButtonPlugin;

impl Plugin for TimingButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update,
                animation,
            ).run_if(in_state(AppState::Ingame)))
            .add_systems(OnEnter(AppState::Mainmenu), despawn)
        ;
    }
}
