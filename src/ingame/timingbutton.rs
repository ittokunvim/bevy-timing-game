use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    WINDOW_SIZE,
    CURSOR_RANGE,
    PATH_IMAGE_TIMINGBUTTON,
    AppState,
};
use crate::ingame::TimingEvent;
use crate::ingame::bar::GRID_SIZE;

const IMAGE_SIZE: u32 = 64;
const SIZE: f32 = 64.0;

#[derive(Default, Component, Debug)]
struct TimingButton {
    pushed: bool,
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    println!("timingbutton: setup");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(IMAGE_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = TimingButton { pushed: false, first: 0, last: 1 };
    let timingbtn_pos = Vec3::new(
        WINDOW_SIZE.x / 2.0,
        WINDOW_SIZE.y / 2.0 - GRID_SIZE * 2.0,
        5.0
    );

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(SIZE)),
                ..Default::default()
            },
            texture: asset_server.load(PATH_IMAGE_TIMINGBUTTON),
            transform: Transform::from_translation(timingbtn_pos),
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
    mut query: Query<(&Transform, &mut TimingButton, &mut TextureAtlas), With<TimingButton>>,
    mut timing_events: EventWriter<TimingEvent>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    let window = window_query.single();
    let mut cursor_pos = window.cursor_position().unwrap();
    let Ok((transform, mut prop, mut atlas)) = query.get_single_mut() else { return };
    let timingbtn_pos = transform.translation.truncate();
    cursor_pos = Vec2::new(cursor_pos.x, -cursor_pos.y + WINDOW_SIZE.y);

    let distance = cursor_pos.distance(timingbtn_pos);

    if distance < SIZE - CURSOR_RANGE {
        timing_events.send_default();
        prop.pushed = true;
        println!("timingbutton: toggled");
        atlas.index = prop.last;
     }
}

fn animation(
    time: Res<Time>,
    mut query: Query<(&mut TimingButton, &mut AnimationTimer, &mut TextureAtlas), With<TimingButton>>,
) {
    let Ok((mut prop, mut timer, mut atlas)) = query.get_single_mut() else { return };

    if !prop.pushed { return }

    timer.tick(time.delta());
    if timer.just_finished() {
        prop.pushed = false;
        atlas.index = prop.first;
    }
}

pub struct TimingButtonPlugin;

impl Plugin for TimingButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ingame)))
            .add_systems(Update, animation.run_if(in_state(AppState::Ingame)))
        ;
    }
}
