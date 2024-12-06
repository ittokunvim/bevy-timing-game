use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    AppState,
    Score,
};

use crate::ingame::{
    PerfectEvent,
    GoodEvent,
    OkEvent,
    BadEvent,
    GameTimer,
};

const SCORE_TEXT: &str = "Score: ";
const TIME_TEXT: &str = " | Time: ";
const PERFECT_POINT: usize = 3;
const GOOD_POINT: usize = 2;
const OK_POINT: usize = 1;
const BAD_POINT: usize = 3;
const TEXT_SIZE: f32 = 24.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_PADDING: Val = Val::Px(5.0);

#[derive(Component)]
struct ScoreboardUi;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ldtk_project_entities: Query<&Handle<LdtkProject>>,
) {
    // Ldtk project
    if !ldtk_project_entities.is_empty() { return }

    println!("scoreboard: setup");
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                SCORE_TEXT,
                TextStyle {
                    font: asset_server.load(PATH_FONT_BOLD),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }),
            TextSection::new(
                TIME_TEXT,
                TextStyle {
                    font: asset_server.load(PATH_FONT_BOLD),
                    font_size: TEXT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: TEXT_PADDING,
            left: TEXT_PADDING,
            ..Default::default()
        }),
        ScoreboardUi,
    ))
    .insert(Name::new("scoreboard"));
} 

fn update(
    mut query: Query<&mut Text, With<ScoreboardUi>>,
    score: Res<Score>,
    timer: ResMut<GameTimer>,
) {
    let mut text = query.single_mut();
    // write score and timer
    text.sections[1].value = score.to_string();
    text.sections[3].value = timer.0.remaining_secs().round().to_string();
}

fn score_points(
    mut score: ResMut<Score>,
    mut perfect_events: EventReader<PerfectEvent>,
    mut good_events: EventReader<GoodEvent>,
    mut ok_events: EventReader<OkEvent>,
    mut bad_events: EventReader<BadEvent>,
) {
    if !perfect_events.is_empty() {
        println!("scoreboard: score increase {}", PERFECT_POINT);
        perfect_events.clear();
        **score += PERFECT_POINT;
    }
    if !good_events.is_empty() {
        println!("scoreboard: score increase {}", GOOD_POINT);
        good_events.clear();
        **score += GOOD_POINT;
    }
    if !ok_events.is_empty() {
        println!("scoreboard: score increase {}", OK_POINT);
        ok_events.clear();
        **score += OK_POINT;
    }
    if !bad_events.is_empty() {
        println!("scoreboard: score decrease {}", PERFECT_POINT);
        bad_events.clear();
        if **score > BAD_POINT { **score -= BAD_POINT } else { **score = 0 };
    }
}

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Ingame), setup)
            .add_systems(Update, (
                update,
                score_points,
            ).run_if(in_state(AppState::Ingame)))
        ;
    }
}
