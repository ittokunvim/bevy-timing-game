use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    AppState,
    Score,
};

const FONT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_GAP: f32 = 64.0;
const GAMEOVER_TEXT: &str = "Game Over...";
const GAMECLEAR_TEXT: &str = "Game Clear!!";
const GAMEOVER_FONT_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "Score: ";
const SCORE_FONT_SIZE: f32 = 24.0;
const RESTART_TEXT: &str = "Click to Restart";
const RESTART_FONT_SIZE: f32 = 24.0;
const BG_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BG_SIZE: Vec2 = Vec2::new(240.0, 240.0);

#[derive(Component)]
pub struct Gameover;

pub fn gameover_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    // Gameover
    let gameover_text = if **score <= 10 { GAMEOVER_TEXT } else { GAMECLEAR_TEXT };

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
            format!("{}{}", SCORE_TEXT, score.to_string()),
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
            RESTART_TEXT,
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
        // Moved app state to ingame
        app_state.set(AppState::Ingame);
    }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), gameover_setup)
            .add_systems(Update, gameover_update.run_if(in_state(AppState::Gameover)));
 
    }
}
