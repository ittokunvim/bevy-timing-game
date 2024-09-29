use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    WINDOW_SIZE,
    FONT_BOLD_PATH,
    FONT_MEDIUM_PATH,
    LDTK_PROJECT_PATH,
    DECIDE_SOUND_PATH,
    AppState,
};

const GRID_SIZE: i32 = 16;

pub const GAMETIME_LIMIT: f32 = 10.0;

const CUE_SPEED: f32 = 7.0;

const BAR_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const BAR_SIZE: Vec2 = Vec2::new((GRID_SIZE * 32) as f32, (GRID_SIZE * 2) as f32);

const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SCOREBOARD_PADDING: Val = Val::Px(5.0);

#[derive(Default, Component, Debug)]
pub struct Cue {
    toggle_move: bool,
}

#[derive(Default, Component)]
pub struct Bar;

#[derive(Default, Bundle, LdtkEntity)]
pub struct CueBundle {
    cue: Cue,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
}

#[derive(Event, Default)]
pub struct DecideEvent;

#[derive(Resource, Deref)]
pub struct DecideSound(Handle<AudioSource>);

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
pub struct ScoreboardUi;

#[derive(Resource)]
pub struct GameTimer(pub Timer);

pub fn ingame_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(LDTK_PROJECT_PATH),
        ..Default::default()
    });

    // Camera
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = WINDOW_SIZE.x / 2.0;
    camera_transform.translation.y = WINDOW_SIZE.y / 2.0;
    // Sound
    let cue_decide_sound = asset_server.load(DECIDE_SOUND_PATH);
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
                "Score: ", 
                TextStyle {
                    font: asset_server.load(FONT_BOLD_PATH),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_MEDIUM_PATH),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCOREBOARD_COLOR,
            }),
            TextSection::new(
                " | Time: ", 
                TextStyle {
                    font: asset_server.load(FONT_BOLD_PATH),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(FONT_MEDIUM_PATH),
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

pub fn ingame_update() {}

pub fn cue_movement(
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

pub fn decide_timing(
    mouse_event: Res<ButtonInput<MouseButton>>,
    cue_query: Query<&Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut decide_events: EventWriter<DecideEvent>,
    mut score: ResMut<Score>,
) {
    if !mouse_event.just_pressed(MouseButton::Left) { return }

    decide_events.send_default();

    let bar_transform = bar_query.single();
    let bar_x = bar_transform.translation.x; // 320
    let cue_transform = cue_query.single();
    let cue_x = cue_transform.translation.x;

    if cue_x < bar_x + GRID_SIZE as f32 && cue_x > bar_x - GRID_SIZE as f32 {
        println!("bar_x: {}, cue_x: {}", bar_x, cue_x);
        **score += 3;
    }
    else if cue_x < bar_x + (GRID_SIZE * 2) as f32 && cue_x > bar_x - (GRID_SIZE * 2) as f32 {
        println!("bar_x: {}, cue_x: {}", bar_x, cue_x);
        **score += 2;
    }
    else {
        println!("bar_x: {}, cue_x: {}", bar_x, cue_x);
        if **score > 0 { **score -= 1 };
    }
}

pub fn play_decide_sound(
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

pub fn update_scoreboard(
    score: Res<Score>,
    timer: ResMut<GameTimer>,
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
) {
    let mut text = scoreboard_query.single_mut();
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

pub fn update_gametimer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        timer.0.reset();
        app_state.set(AppState::Gameover);
    }
}
