use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_TIMINGBUTTON,
    PATH_LDTK_PROJECT,
};

use crate::ingame::{
    GRID_SIZE,
    TIMINGBTN_SIZE,
    BAR_SIZE,
    Bar,
    TimingButton,
    AnimationTimer,
};

const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);

pub fn component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(PATH_LDTK_PROJECT),
        ..Default::default()
    })
    .insert(Name::new("ldtk"));
    // Camera
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = WINDOW_SIZE.x / 2.0;
    camera_transform.translation.y = WINDOW_SIZE.y / 2.0;
    // Bar
    let bar_y = WINDOW_SIZE.y - (GRID_SIZE * 4) as f32 - BAR_SIZE.y / 2.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BAR_COLOR,
                custom_size: Some(BAR_SIZE),
                ..default()
            },
            transform: Transform::from_xyz(WINDOW_SIZE.x / 2.0, bar_y, 0.0),
            ..default()
        },
        Bar,
    ))
    .insert(Name::new("bar"));
    // Timing button
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(TIMINGBTN_SIZE), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = TimingButton { pushed: false, first: 0, last: 1 };
    let timingbtn_pos = Vec3::new(
        WINDOW_SIZE.x / 2.0,
        WINDOW_SIZE.y / 2.0 - GRID_SIZE as f32 * 2.0,
        5.0
    );

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PATH_IMAGE_TIMINGBUTTON),
            transform: Transform::from_translation(timingbtn_pos),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating))
    ))
    .insert(Name::new("timingbtn"));
}
