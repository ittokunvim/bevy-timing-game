use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    WINDOW_SIZE,
    PATH_FONT_MEDIUM,
    PATH_FONT_BOLD,
    AppState,
    Config,
    Score,
};

const GAMECLEAR_TEXT: &str = "Game Clear";
const GAMECLEAR_SIZE: f32 = 32.0;
const SCORE_TEXT: &str = "Score: ";
const RETRY_TEXT: &str = "Retry: Key[R]";
const BACKTOTITLE_TEXT: &str = "Back to Title: Key[B]";
const BOARD_SIZE: Vec2 = Vec2::new(240.0, 240.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
pub struct Gameclear;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    println!("gameclear: setup");
    // gameclear
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMECLEAR_SIZE / 2.0 - TEXT_PADDING * 1.5);

    commands.spawn((
        TextBundle::from_section(
            GAMECLEAR_TEXT,
            TextStyle {
                font: asset_server.load(PATH_FONT_BOLD),
                font_size: GAMECLEAR_SIZE,
                color: TEXT_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            top,
            justify_self: JustifySelf::Center,
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("gameclear"));
    // score
    let text = format!("{}{}", SCORE_TEXT, score.to_string());
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 - TEXT_PADDING * 0.5);

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
        Gameclear,
    ))
    .insert(Name::new("score"));
    // retry
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 0.5);

    commands.spawn((
        TextBundle::from_section(
            RETRY_TEXT,
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
        Gameclear,
    ))
    .insert(Name::new("retry"));
    // back to title
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING * 1.5);

    commands.spawn((
        TextBundle::from_section(
            BACKTOTITLE_TEXT, 
            TextStyle {
                font: asset_server.load(PATH_FONT_MEDIUM),
                font_size: TEXT_SIZE,
                color: TEXT_COLOR,
            }
        )
        .with_style(Style {
            position_type: PositionType::Relative,
            justify_self: JustifySelf::Center,
            top,
            ..Default::default()
        }),
        Gameclear,
    ))
    .insert(Name::new("backtotitle"));
    // board
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
            material: materials.add(BOARD_COLOR),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        },
        Gameclear,
    ))
    .insert(Name::new("board"));
}

fn update(
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |app_state: AppState| {
        println!("gameclear: config setup ingame is false");
        config.setup_ingame = false;
        println!("gameclear: moved state to {:?} from Gameclear", app_state);
        next_state.set(app_state);
    };

    for key in keyboard_input.get_just_pressed() {
        match key {
            KeyCode::KeyR => closure(AppState::Ingame),
            KeyCode::KeyB => closure(AppState::Mainmenu),
            _ => {},
        }
    }
}

fn despawn(
    mut commands: Commands,
    query: Query<Entity, With<Gameclear>>,
) {
    println!("gameclear: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct GameclearPlugin;

impl Plugin for GameclearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameclear), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameclear)))
            .add_systems(OnExit(AppState::Gameclear), despawn)
        ;
    }
}
