use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    AppState,
    Score,
};

use crate::ingame::{
    GRID_SIZE,
    Cue,
    Bar,
    TimingEvent,
    GameTimer,
};

const SCOREBOARD_FONT_SIZE: f32 = 24.0;
const SCOREBOARD_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const SCOREBOARD_PADDING: Val = Val::Px(5.0);
const SCOREBOARD_SCORE_TEXT: &str = "Score: ";
const SCOREBOARD_TIME_TEXT: &str = " | Time: ";

#[derive(Component)]
struct ScoreboardUi;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

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
} 

fn write_scoreboard(
    mut scoreboard_query: Query<&mut Text, With<ScoreboardUi>>,
    score: Res<Score>,
    timer: ResMut<GameTimer>,
) {
    let mut text = scoreboard_query.single_mut();
    // write score and timer
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

fn score_points(
    mut timing_events: EventReader<TimingEvent>,
    cue_query: Query<&Transform, (With<Cue>, Without<Bar>)>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut score: ResMut<Score>,
) {
    if timing_events.is_empty() { return }
    timing_events.clear();

    let cue_x = cue_query.single().translation.x;
    let bar_x = bar_query.single().translation.x;

    // if cue was just timing
    if cue_x < bar_x + GRID_SIZE as f32 && cue_x > bar_x - GRID_SIZE as f32 {
        **score += 3;
    }
    // if cue was good timing
    else if cue_x < bar_x + (GRID_SIZE * 2) as f32 && cue_x > bar_x - (GRID_SIZE * 2) as f32 {
        **score += 2;
    }
    else {
        if **score > 0 { **score -= 1 };
    }
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                write_scoreboard,
                score_points,
            ).run_if(in_state(AppState::Ingame)));
    }
}
