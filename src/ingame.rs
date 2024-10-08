use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    PATH_FONT_BOLD,
    PATH_FONT_MEDIUM,
    PATH_LDTK_PROJECT,
    PATH_SOUND_DECIDE,
    AppState,
    Score,
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

#[derive(Resource)]
struct GameTimer(Timer);

fn ingame_setup(
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
    });

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
    ));
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
    ));
}

fn cue_movement(
    mut cue_query: Query<(&mut Transform, &mut Cue), With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
) {
    let bar_transform = bar_query.single();
    let bar_x = bar_transform.translation.x;

    for (mut cue_transform, mut cue) in &mut cue_query {
        let cue_x = cue_transform.translation.x;

        if cue_x > bar_x + BAR_SIZE.x / 2.0 || cue_x < bar_x - BAR_SIZE.x / 2.0 {
            cue.toggle_move = !cue.toggle_move;
        }
        cue_transform.translation.x += if cue.toggle_move { CUE_SPEED } else { -CUE_SPEED };
    }
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

    let bar_transform = bar_query.single();
    let bar_x = bar_transform.translation.x;
    let cue_transform = cue_query.single();
    let cue_x = cue_transform.translation.x;

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
            .add_systems(OnEnter(AppState::Ingame), ingame_setup)
            .add_systems(Update, (
                cue_movement,
                decide_timing,
                play_decide_sound,
                update_scoreboard,
                update_gametimer,
            ).run_if(in_state(AppState::Ingame)));
    }
}
