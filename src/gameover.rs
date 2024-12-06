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
};

const GAMEOVER_TEXT: &str = "Game Over";
const GAMEOVER_SIZE: f32 = 32.0;
const RETRY_TEXT: &str = "Retry: Key[R]";
const BACKTOTITLE_TEXT: &str = "Back to Title: Key[B]";
const BOARD_SIZE: Vec2 = Vec2::new(240.0, 240.0);
const BOARD_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_SIZE: f32 = 20.0;
const TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_PADDING: f32 = 50.0;

#[derive(Component)]
pub struct Gameover;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("gameover: setup");
    // gameover
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - GAMEOVER_SIZE / 2.0 - TEXT_PADDING);

    commands.spawn((
        TextBundle::from_section(
            GAMEOVER_TEXT,
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
    // retry
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0);

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
        Gameover,
    ))
    .insert(Name::new("retry"));
    // back to title
    let top = Val::Px(WINDOW_SIZE.y / 2.0 - TEXT_SIZE / 2.0 + TEXT_PADDING);

    commands.spawn((
        TextBundle::from_section(
            BACKTOTITLE_TEXT,
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
    .insert(Name::new("backtotitle"));
    // board
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(BOARD_SIZE.x, BOARD_SIZE.y))),
            material: materials.add(BOARD_COLOR),
            ..Default::default()
        },
        Gameover,
    ))
    .insert(Name::new("board"));
}

fn update(
    mut config: ResMut<Config>,
    mut next_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut closure = |app_state: AppState| {
        println!("gameover: change config.setup_ingame to true");
        config.setup_ingame = true;
        println!("gameover: moved state to {:?} from Gameover", app_state);
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
    query: Query<Entity, With<Gameover>>,
) {
    println!("gameover: despawn");
    for entity in query.iter() { commands.entity(entity).despawn() }
}

pub struct GameoverPlugin;

impl Plugin for GameoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Gameover), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Gameover)))
            .add_systems(OnExit(AppState::Gameover), despawn)
        ;
    }
}
