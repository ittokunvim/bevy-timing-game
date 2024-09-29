use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
};

use crate::ingame::{
    BAR_SIZE,
    Cue,
    Bar,
    Score,
};

const GAMEOVER_FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BG_SIZE: Vec2 = Vec2::new(240.0, 240.0);
const TEXT_GAP: f32 = 64.0;
const SCORE_FONT_SIZE: f32 = 24.0;
const RESTART_FONT_SIZE: f32 = 24.0;

#[derive(Component)]
pub struct Gameover;

pub fn gameover_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    // Gameover
    let gameover_text = if **score <= 10 { "Game Over..." } else { "Game Clear!!!" };

    commands.spawn((
        TextBundle::from_section(
            gameover_text,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: GAMEOVER_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_FONT_SIZE / 2.0 - TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Score
    commands.spawn((
        TextBundle::from_section(
            format!("Score: {}", score.to_string()),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCORE_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - RESTART_FONT_SIZE / 2.0),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Click to Restart
    commands.spawn((
        TextBundle::from_section(
            "Click to Restart",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: RESTART_FONT_SIZE,
                color: FONT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(WINDOW_SIZE.y / 2.0 - RESTART_FONT_SIZE / 2.0 + TEXT_GAP),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        Gameover,
    ));
    // Gameover background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(BG_SIZE),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(
                    WINDOW_SIZE.x / 2.0,
                    WINDOW_SIZE.y / 2.0,
                    10.0
                ),
                ..default()
            },
            ..default()
        },
        Gameover,
    ));
}

pub fn gameover_update(
    mouse_event: Res<ButtonInput<MouseButton>>,
    gameover_query: Query<Entity, With<Gameover>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut cue_query: Query<&mut Transform, With<Cue>>,
    bar_query: Query<&Transform, (With<Bar>, Without<Cue>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // Mouse clicked
    if mouse_event.just_pressed(MouseButton::Left) {
        // Despawned gameover entities
        for gameover_entity in gameover_query.iter() {
            commands.entity(gameover_entity).despawn();
        }
        // Reset score
        **score = 0;
        // Reset cue position
        let bar_transform = bar_query.single();

        for mut cue_transform in &mut cue_query {
            cue_transform.translation.x = bar_transform.translation.x + BAR_SIZE.x / 2.0;
        }
        // Moved app state to ingame
        app_state.set(AppState::Ingame);
    }
}
