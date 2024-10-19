use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_PAUSE_IMAGE,
    PATH_FONT_BOLD,
    PATH_FONT_MEDIUM,
    PATH_LDTK_PROJECT,
    PATH_SOUND_DECIDE,
    AppState,
    Score,
};

use crate::pause::{
    PauseButton,
    update_pausebtn,
    get_pausebtn,
};

const GRID_SIZE: i32 = 16;
const GAMETIME_LIMIT: f32 = 10.0;
const CUE_SPEED: f32 = 7.0;
const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
const BAR_SIZE: Vec2 = Vec2::new((GRID_SIZE * 32) as f32, (GRID_SIZE * 2) as f32);
const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SCOREBOARD_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_SCORE_TEXT: &str = "Score: ";
const SCOREBOARD_TIME_TEXT: &str = " | Time: ";
const MAX_EFFECT_CAPACITY: u32 = 4096;

#[derive(Default, Component, Debug)]
struct Cue {
    toggle_move: bool,
}

#[derive(Default, Component)]
struct Bar;

#[derive(Default, Bundle, LdtkEntity)]
struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

#[derive(Event, Default)]
struct DecideEvent;

#[derive(Resource, Deref)]
struct DecideSound(Handle<AudioSource>);

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct DecideEffect;

#[derive(Component)]
struct ReversalEffect;

#[derive(Resource)]
struct GameTimer(Timer);

fn setup(
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
    // Sound
    let cue_decide_sound = asset_server.load(PATH_SOUND_DECIDE);
    commands.insert_resource(DecideSound(cue_decide_sound));
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
        get_pausebtn(asset_server, PATH_PAUSE_IMAGE.to_string()),
        PauseButton,
    ))
    .insert(Name::new("pausebtn"));
}

fn effect_setup(
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut commands: Commands,
) {
    if !ldtk_project_entities.is_empty() { return }

    // Decide effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 0.7, 0.0, 1.0));
    gradient.add_key(1.0, Vec4::new(0.0, 0.7, 0.0, 0.0));
    let writer = ExprWriter::new();
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
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
    let effect = EffectAsset::new(vec![MAX_EFFECT_CAPACITY], Spawner::once(1000.0.into(), true), writer.finish())
        .with_name("decide_effect")
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
        DecideEffect,
    ))
    .insert(Name::new("decide_effect"));
    // Reversal effect
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.7, 0.0, 0.0, 1.0));
    gradient.add_key(0.8, Vec4::new(0.7, 0.0, 0.0, 0.5));
    gradient.add_key(1.0, Vec4::new(0.7, 0.0, 0.0, 0.0));
    let writer = ExprWriter::new();
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(1.0).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
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

fn cue_movement(
    mut cue_query: Query<(&mut Transform, &mut Cue), With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<ReversalEffect>, Without<Cue>, Without<Bar>)>,
) {
    let Ok((mut cue_transform, mut cue)) = cue_query.get_single_mut() else { return; };
    let cue_x = cue_transform.translation.x;
    let bar_transform = bar_query.single();
    let (bar_x, bar_y) = (bar_transform.translation.x, bar_transform.translation.y);

    if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
        // reversal movement
        cue.toggle_move = !cue.toggle_move;
        // reversal effect
        let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
        let transform_x = match cue.toggle_move {
            true => bar_x - BAR_SIZE.x / 2.0,
            false => bar_x + BAR_SIZE.x / 2.0,
        };
        let rotation_z = if cue.toggle_move { 1.5 } else { -1.5 };
        effect_transform.translation = Vec3::new(transform_x, bar_y, 0.0);
        effect_transform.rotation = Quat::from_rotation_z(rotation_z);
        spawner.reset();
    }

    cue_transform.translation.x += if cue.toggle_move { CUE_SPEED } else { -CUE_SPEED };
}

fn decide_timing(
    mouse_event: Res<ButtonInput<MouseButton>>,
    cue_query: Query<&Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut decide_events: EventWriter<DecideEvent>,
    mut score: ResMut<Score>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return }

    decide_events.send_default();

    let bar_x = bar_query.single().translation.x;
    let cue_x = cue_query.single().translation.x;

    if cue_x < bar_x + GRID_SIZE as f32 && cue_x > bar_x - GRID_SIZE as f32 {
        **score += 3;
    }
    else if cue_x < bar_x + (GRID_SIZE * 2) as f32 && cue_x > bar_x - (GRID_SIZE * 2) as f32 {
        **score += 2;
    }
    else {
        if **score > 0 { **score -= 1 };
    }
}

fn play_decide_sound(
    mut commands: Commands,
    mut decide_events: EventReader<DecideEvent>,
    sound: Res<DecideSound>,
) {
    if decide_events.is_empty() { return }

    decide_events.clear();
    commands.spawn(AudioBundle {
        source: sound.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
}

fn spawn_decide_effect(
    mut effect: Query<(&mut EffectSpawner, &mut Transform), (With<DecideEffect>, Without<Cue>, Without<Bar>)>,
    mut decide_events: EventReader<DecideEvent>,
    cue_query: Query<&Transform, With<Cue>>,
) {
    if decide_events.is_empty() { return }

    let Ok((mut spawner, mut effect_transform)) = effect.get_single_mut() else { return; };
    let cue_transform = cue_query.single();

    decide_events.clear();
    effect_transform.translation = cue_transform.translation;
    spawner.reset();
}

fn update_scoreboard(
    score: Res<Score>,
    timer: ResMut<GameTimer>,
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

fn update_gametimer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut cue_query: Query<&mut Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Reset timer
        timer.0.reset();
        // Reset cue position
        let bar_transform = bar_query.single();

        for mut cue_transform in &mut cue_query {
            cue_transform.translation.x = bar_transform.translation.x + BAR_SIZE.x / 2.0;
        }
        // Move app state
        app_state.set(AppState::Gameover);
    }
}

pub struct IngamePlugin;

impl Plugin for IngamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DecideEvent>()
            .insert_resource(GameTimer(Timer::from_seconds(GAMETIME_LIMIT, TimerMode::Once)))
            .register_ldtk_entity::<CueBundle>("Cue")
            .add_systems(OnEnter(AppState::Ingame), (
                setup,
                effect_setup,
            ))
            .add_systems(Update, (
                cue_movement,
                decide_timing,
                play_decide_sound,
                spawn_decide_effect,
                update_scoreboard,
                update_gametimer,
                update_pausebtn,
            ).run_if(in_state(AppState::Ingame)));
    }
}
