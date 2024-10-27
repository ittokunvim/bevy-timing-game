use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    PATH_IMAGE_PAUSE,
    PATH_IMAGE_TIMINGBUTTON,
    PATH_LDTK_PROJECT,
    PATH_SOUND_TIMING,
    PATH_SOUND_REVERSAL,
};

use crate::pause::{
    PauseButton,
    get_pausebtn,
};

use crate::ingame::{
    GRID_SIZE,
    TIMINGBTN_SIZE,
    BAR_SIZE,
    Bar,
    TimingButton,
    AnimationTimer,
    ScoreboardUi,
    TimingSound,
    ReversalSound,
};

const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SCOREBOARD_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_SCORE_TEXT: &str = "Score: ";
const SCOREBOARD_TIME_TEXT: &str = " | Time: ";

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
    // Sounds
    let cue_timing_sound = asset_server.load(PATH_SOUND_TIMING);
    let cue_reversal_sound = asset_server.load(PATH_SOUND_REVERSAL);
    commands.insert_resource(TimingSound(cue_timing_sound));
    commands.insert_resource(ReversalSound(cue_reversal_sound));
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
    // Scoreboard
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                SCOREBOARD_SCORE_TEXT,
                TextStyle {
                    font: asset_server.load(PATH_FONT_BOLD),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_COLOR,
            }),
            TextSection::new(
                SCOREBOARD_TIME_TEXT,
                TextStyle {
                    font: asset_server.load(PATH_FONT_BOLD),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_PADDING,
            left: SCOREBOARD_PADDING,
            ..default()
        }),
        ScoreboardUi,
    ))
    .insert(Name::new("scoreboard"));
    // Pause button
    commands.spawn((
        get_pausebtn(asset_server, PATH_IMAGE_PAUSE.to_string()),
        PauseButton,
    ))
    .insert(Name::new("pausebtn"));
}
