use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    WINDOW_SIZE,
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    AppState,
    Score,
};

const GAMECLEAR_TEXT: &str = "Game Clear!!";
const GAMEOVER_TEXT: &str = "Game Over...";
const GAMEOVER_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "Score: ";
const RESTART_TEXT: &str = "Click to Restart";
const BOARD_SIZE: Vec2 = Vec2::new(240.0, 240.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_PADDING: f32 = 64.0;

#[derive(Component)]
pub struct Gameover;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    println!("gameover: setup");
    // gameover
    let text = if **score >= 10 { GAMECLEAR_TEXT } else { GAMEOVER_TEXT };
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_SIZE / 2.0 - TEXT_PADDING);

    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load(PATH_FONT_BOLD),
                font_size: GAMEOVER_SIZE,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top,
            justify_self: JustifySelf::Center,
                ..Default::default()
        }),
        Gameover,
    ))
    .insert(Name::new("gameover"));
    // score
    let text = format!("{}{}", SCORE_TEXT, score.to_string());
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0);
    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top,
            ..Default::default()
        }),
        Gameover,
    ))
    .insert(Name::new("score"));
    // restart
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING);
    commands.spawn((
        TextBundle::from_section(
            RESTART_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top,
            ..Default::default()
        }),
        Gameover,
    ))
    .insert(Name::new("restart"));
    // board
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
            material: materials.add(BOARD_COLOR),
            transform: Transform::from_xyz(
                WINDOW_SIZE.x / 2.0, 
                WINDOW_SIZE.y / 2.0, 
                10.0,
            ),
            ..Default::default()
        },
        Gameover,
    ))
    .insert(Name::new("board"));
}

pub fn update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut score: ResMut<Score>,
    mouse_events: Res<ButtonInput<MouseButton>>,
    query: Query<Entity, With<Gameover>>,
) {
    if !mouse_events.just_pressed(MouseButton::Left) { return }

    println!("gameover: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
    println!("gameover: reset score");
    **score = 0;
    println!("gameover: moved state to Ingame from Gameover");
    next_state.set(AppState::Ingame);
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
        ;
    }
}
