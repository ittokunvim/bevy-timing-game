use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_IMAGE_PAUSE,
    PATH_FONT_BOLD,
    PATH_FONT_MEDIUM,
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
    BAR_SIZE,
    Bar,
    ScoreboardUi,
    TimingEffect,
    TimingSound,
    ReversalEffect,
    ReversalSound,
};

const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SCOREBOARD_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_SCORE_TEXT: &str = "Score: ";
const SCOREBOARD_TIME_TEXT: &str = " | Time: ";
const MAX_EFFECT_CAPACITY: u32 = 4096;

pub fn component(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
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

pub fn effect(
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    if !ldtk_project_entities.is_empty() { return }

    // Timing effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.7, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.7, 0.0, 0.0));
    let writer = ExprWriter::new();
    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(5.0).expr(),
        dimension: ShapeDimension::Surface,
    };
    let init_vel = SetVelocityCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        speed: writer.lit(10.0).expr(),
    };
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let effect = EffectAsset::new(vec![MAX_EFFECT_CAPACITY], Spawner::once(1000.0.into(), true), writer.finish())
        .with_name("timing_effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .render(SizeOverLifetimeModifier {
            gradient: Gradient::constant(Vec2::splat(1.0)),
            screen_space_size: false,
        })
        .render(ColorOverLifetimeModifier { gradient });
    let effect_handle = effects.add(effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(10.0)),
            ..default()
        },
        TimingEffect,
    ))
    .insert(Name::new("timing_effect"));
    // Reversal effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.7, 0.0, 0.0, 1.0));
    gradient.add_key(0.8, Vec4::new(0.7, 0.0, 0.0, 0.5));
    gradient.add_key(1.0, Vec4::new(0.7, 0.0, 0.0, 0.0));
    let writer = ExprWriter::new();
    let init_pos = SetPositionCone3dModifier {
        base_radius: writer.lit(2.0).expr(),
        top_radius: writer.lit(10.0).expr(),
        height: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(20.0).expr(),
    };
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
    let size = (writer.rand(ScalarType::Float) * writer.lit(1.5) + writer.lit(1.5)).expr();
    let init_size = SetAttributeModifier::new(Attribute::SIZE, size);
    let effect = EffectAsset::new(vec![MAX_EFFECT_CAPACITY], Spawner::once(50.0.into(), false), writer.finish())
        .with_name("reversal_effect")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(init_size)
        .render(ColorOverLifetimeModifier { gradient });
    let effect_handle = effects.add(effect);

    commands.spawn((
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect_handle).with_z_layer_2d(Some(10.0)),
            ..default()
        },
        ReversalEffect,
    ))
    .insert(Name::new("reversal_effect"));
}
